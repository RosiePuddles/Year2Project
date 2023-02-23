use std::{
	cell::Cell,
	fmt::Display,
	fs::File,
	future::{ready, Ready},
	io::Write,
};

use actix_web::{body::EitherBody, dev::{Service, ServiceRequest, ServiceResponse, Transform}, http::Method, web, Error, HttpMessage};
use actix_web::dev::Payload;
use actix_web::web::BytesMut;
use chrono::Local;
use futures_util::future::LocalBoxFuture;
use futures_util::{Stream, StreamExt, TryFutureExt, TryStreamExt};

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

	fn call(&self, mut req: ServiceRequest) -> Self::Future {
		let req_inner = req.request();
		let logger = req_inner.app_data::<web::Data<Logger>>().unwrap();
		let host = req_inner.connection_info().clone();
		let host = host.host();
		let method = req.method().clone();
		let path = req_inner.path();
		if let Err(e) = logger.request(host, &method, path, req_inner.content_type()) {
			println!("Unable to write to log file! {}", e)
		}

		#[cfg(not(debug_assertions))]
		if path.starts_with("/api/")
			&& req_inner.cookie("API_KEY").map(|c| c.to_string())
				!= Some(env!("API_KEY", "No API key given!").to_string())
		{
			if let Err(e) = logger.response(host, &method, path, 403) {
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
			if let Err(e) = logger.response(host, &method, path, res.status().as_u16()) {
				println!("Unable to write to log file! {}", e)
			}
			Ok(res)
		})
	}
}

pub struct Logger<'a> {
	f: Cell<File>,
	fmt: &'a str,
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

impl<'a> Logger<'a> {
	pub fn default(f: File, print: bool) -> Self {
		Self {
			f: Cell::new(f),
			fmt: "%+",
			print,
		}
	}

	fn write<T: ToString>(&self, msg: T) -> std::io::Result<()> {
		let f_ptr = self.f.as_ptr();
		let mut f_ref = unsafe { f_ptr.as_ref().unwrap() };
		f_ref.write(msg.to_string().as_bytes())?;
		Ok(())
	}

	pub fn request(&self, host: &str, method: &Method, path: &str, content_type: &str) -> std::io::Result<()> {
		let printed = format!(
			"[{} REQ  ] {} {} {} {}\n",
			Local::now().format(self.fmt),
			host,
			method,
			path,
			content_type
		);
		if self.print {
			print!("{}", printed)
		}
		self.write(printed)
	}

	pub fn response(&self, host: &str, method: &Method, path: &str, code: u16) -> std::io::Result<()> {
		let printed = format!(
			"[{} RES  ] {} {} {} {}\n",
			Local::now().format(self.fmt),
			host,
			method,
			path,
			code
		);
		if self.print {
			print!("{}", printed)
		}
		self.write(printed)
	}

	pub fn info<T: Display>(&self, message: T) -> std::io::Result<()> {
		let printed = format!("[{} INFO ] {}\n", Local::now().format(self.fmt), message);
		if self.print {
			print!("{}", printed)
		}
		self.write(printed)
	}

	pub fn error<T: Display>(&self, message: T) -> std::io::Result<()> {
		let printed = format!("[{} ERROR] {}\n", Local::now().format(self.fmt), message);
		if self.print {
			print!("{}", printed)
		}
		self.write(printed)
	}

	pub fn warn<T: Display>(&self, message: T) -> std::io::Result<()> {
		let printed = format!("[{} WARN ] {}\n", Local::now().format(self.fmt), message);
		if self.print {
			print!("{}", printed)
		}
		self.write(printed)
	}
}
