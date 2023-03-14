mod api;
mod db;
mod front;
mod logger;

use ::config::{Config, Environment};
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use tokio_postgres::NoTls;

/// Simple wrapper around a logger call
#[macro_export]
macro_rules! logger_wrap {
	($l:ident.$m:ident, $r:ident, $s:expr) => {
		if let Err(e) = $l.$m(
			$r.connection_info().host(),
			$r.method(),
			$r.path(),
			$r.content_type(),
			$s,
		) {
			println!("Unable to write to log file {}:{}! {}", file!(), line!(), e)
		}
	};
}

#[macro_export]
macro_rules! paths {
    ($($j:expr),*$(,)?) => {
		pub fn paths<T>(app: App<T>) -> App<T>
		where
			T: ServiceFactory<ServiceRequest, Config = (), Error = Error, InitError = ()>,
		{
			app$(.service($j))*
		}
	};
    (default $d:expr, $($j:expr),*$(,)?) => {
		pub fn paths<T>(app: App<T>) -> App<T>
		where
			T: ServiceFactory<ServiceRequest, Config = (), Error = Error, InitError = ()>,
		{
			app.default_service($d)$(.service($j))*
		}
	};
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	dotenv().ok();

	let config_ = Config::builder().add_source(Environment::default()).build().unwrap();

	let config: db::DbConfig = config_.try_deserialize().unwrap();

	let pool = config.pg.create_pool(None, NoTls).unwrap();
	let log_file = std::fs::OpenOptions::new()
		.write(true)
		.create(true)
		.append(true)
		.open("server.log")?;
	let logger = logger::Logger::default(log_file, cfg!(debug_assertions));

	let server = HttpServer::new(move || {
		let mut app = App::new()
			.wrap(logger::LoggerMiddleware::new())
			.app_data(web::Data::new(pool.clone()))
			.app_data(web::Data::new(logger.clone()));
		for i in [api::paths, front::paths] {
			app = i(app)
		}
		app
	})
	.bind(config.server_addr.clone())?
	.run();
	println!("Server running at http://{}/", config.server_addr);
	server.await
}
