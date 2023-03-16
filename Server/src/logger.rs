//! # Custom logger middleware
//!
//! this provides the logger used by the server. I'm not too sure why it's constructed like it is,
//! so if you want details have a look at the [Actix middleware docs](https://actix.rs/docs/middleware/)

use std::{
	cell::{Cell, Ref},
	fmt::Display,
	fs::File,
	future::{ready, Ready},
	io::Write,
};

use actix_web::{
	body::EitherBody,
	cookie::Cookie,
	dev::{Service, ServiceRequest, ServiceResponse, Transform},
	http::{header::HeaderMap, Method, StatusCode},
	web, Error, HttpMessage, HttpResponse,
};
use chrono::Local;
use futures_util::future::LocalBoxFuture;

use crate::logger_wrap;

pub struct LoggerMiddleware;

impl LoggerMiddleware {
	pub fn new() -> Self { Self }
}

impl<S, B> Transform<S, ServiceRequest> for LoggerMiddleware
where
	S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
	S::Future: 'static,
	B: 'static,
{
	type Error = Error;
	type Future = Ready<Result<Self::Transform, Self::InitError>>;
	type InitError = ();
	type Response = ServiceResponse<EitherBody<B>>;
	type Transform = LoggerMiddlewareTransformed<S>;

	fn new_transform(&self, service: S) -> Self::Future { ready(Ok(LoggerMiddlewareTransformed(service))) }
}

pub struct LoggerMiddlewareTransformed<S>(S);

impl<S, B> Service<ServiceRequest> for LoggerMiddlewareTransformed<S>
where
	S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
	S::Future: 'static,
	B: 'static,
{
	type Error = Error;
	type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;
	type Response = ServiceResponse<EitherBody<B>>;

	fn poll_ready(&self, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), Self::Error>> {
		self.0.poll_ready(cx).map_err(core::convert::Into::into)
	}

	fn call(&self, req: ServiceRequest) -> Self::Future {
		let req_inner = req.request();
		let logger = req_inner.app_data::<web::Data<Logger>>().unwrap();
		let host = req_inner.connection_info().clone();
		let host = host.host();
		let method = req.method().clone();
		let path = req_inner.path();
		if let Err(e) = logger.request(host, &method, path, req_inner.content_type(), req_inner.cookies()) {
			println!("Unable to write to log file! {}", e)
		}

		#[cfg(not(debug_assertions))]
		if path.starts_with("/api/")
			&& req_inner.cookie("API_KEY").map(|c| c.to_string())
				!= Some(env!("API_KEY", "No API key given!").to_string())
		{
			if let Err(e) = logger.response(host, &method, path, StatusCode::FORBIDDEN, res.headers()) {
				println!("Unable to write to log file! {}", e)
			}
			let request = req.request().clone();

			let response = HttpResponse::Forbidden().finish().map_into_right_body();

			return Box::pin(async { Ok(ServiceResponse::new(request, response)) })
		}

		let fut = self.0.call(req);

		Box::pin(async move {
			let res: Self::Response = fut.await.map(ServiceResponse::map_into_left_body)?;
			let logger = res.request().app_data::<web::Data<Logger>>().unwrap();
			let host = res
				.request()
				.head()
				.headers
				.get("host")
				.map(|v| v.to_str().unwrap())
				.unwrap_or("unknown");
			let method = res.request().method().clone();
			let path = res.request().path();
			if let Err(e) = logger.response(host, &method, path, res.status(), res.headers()) {
				println!("Unable to write to log file! {}", e)
			}
			Ok(res)
		})
	}
}

/// Primary logger
///
/// This handles all the loggin agter being constructed by the Actix App
pub struct Logger<'a> {
	/// Cell for the logger file (server.log) to allow writing to a file
	f: Cell<File>,
	/// Datetime format. Defaults to `%+`. See [`crate::format::strftime`] for format codes
	fmt: &'a str,
	/// Print bool. If true it will both print logs to stdout and the log file, otherwise it will
	/// just write to the log file
	print: bool,
}

impl Clone for Logger<'_> {
	fn clone(&self) -> Self {
		let f_ptr = self.f.as_ptr();
		let f_ref = unsafe { f_ptr.as_ref().unwrap() };
		let f = f_ref.try_clone().unwrap();
		Self {
			f: Cell::new(f),
			fmt: self.fmt,
			print: self.print,
		}
	}
}

macro_rules! printer {
	($n:ident) => {
		#[doc = concat!(stringify!($n), " log level")]
		pub fn $n<T: Display>(
			&self, host: &str, method: &Method, path: &str, content_type: &str, message: T,
		) -> std::io::Result<()> {
			let printed = format!(
				"[{} {:<5}] {} {} {} {} {}\n",
				Local::now().format(self.fmt),
				stringify!($n).to_uppercase(),
				host,
				method,
				path,
				content_type,
				message
			);
			if self.print {
				print!("{}", printed)
			}
			self.write(printed)
		}
	};
}

impl<'a> Logger<'a> {
	printer!(info);

	printer!(error);

	printer!(warn);

	pub fn default(f: File, print: bool) -> Self {
		Self {
			f: Cell::new(f),
			fmt: "%+",
			print,
		}
	}

	/// General purpose write function for internal use.\
	/// **Uses unsafe code:**\
	/// To allow writing to a file, we take the file as a `*mut File` then use
	/// `f_ptr.as_ref().unwrap()` in an unsafe block to get a reference to the file. We then write
	/// to the mutable file reference. This was the only way I've found to be able to write to a log
	/// file under an immutable struct.
	fn write<T: ToString>(&self, msg: T) -> std::io::Result<()> {
		let f_ptr = self.f.as_ptr();
		let mut f_ref = unsafe { f_ptr.as_ref().unwrap() };
		f_ref.write(msg.to_string().as_bytes())?;
		Ok(())
	}

	/// Request log. This is used when a request is made and will write general request info.
	pub fn request<CPE: Display>(
		&self, host: &str, method: &Method, path: &str, content_type: &str, cookies: Result<Ref<Vec<Cookie<'_>>>, CPE>,
	) -> std::io::Result<()> {
		let printed = format!(
			"[{} REQ  ] {} {} {} {} [{}]\n",
			Local::now().format(self.fmt),
			host,
			method,
			path,
			content_type,
			match cookies {
				Ok(cj) => cj.iter().map(|c| c.to_string()).collect::<Vec<_>>().join(", "),
				Err(e) => format!("Unable to parse cookies {}", e),
			}
		);
		if self.print {
			print!("{}", printed)
		}
		self.write(printed)
	}

	/// Response log. This is used when a request is responded to and includes useful response
	/// information
	pub fn response(
		&self, host: &str, method: &Method, path: &str, code: StatusCode, headers: &HeaderMap,
	) -> std::io::Result<()> {
		let printed = format!(
			"[{} RES  ] {} {} {} {} [{}]\n",
			Local::now().format(self.fmt),
			host,
			method,
			path,
			code,
			headers
				.iter()
				.map(|(name, val)| format!(
					"'{}':'{}'",
					name,
					val.to_str().unwrap_or(
						&*val
							.as_bytes()
							.iter()
							.fold(String::new(), |acc, b| format!("{}{:x}", acc, b))
					)
				))
				.collect::<Vec<_>>()
				.join(", ")
		);
		if self.print {
			print!("{}", printed)
		}
		self.write(printed)
	}

	/// Cleaner log call. This is made by the [`clean`](crate::front::clean) function when it needs
	/// to log something and is typically an error. A call to this is normally wrapped under a
	/// [`logger_wrap!`](crate::logger_wrap!) call
	pub fn clean<T: Display>(&self, message: T) -> std::io::Result<()> {
		let printed = format!("[{} CLEAN] {}\n", Local::now().format(self.fmt), message);
		if self.print {
			print!("{}", printed)
		}
		self.write(printed)
	}
}
