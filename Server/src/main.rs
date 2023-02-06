mod api;
mod config;
mod logger;

use ::config::{Config, Environment};
use actix_web::{web, App};
use dotenv::dotenv;
use tokio_postgres::NoTls;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	dotenv().ok();

	let config_ = Config::builder().add_source(Environment::default()).build().unwrap();

	let config: api::db::ExampleConfig = config_.try_deserialize().unwrap();

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
		for i in [api::paths] {
			app = i(app)
		}
		app
	})
	.bind(config.server_addr.clone())?
	.run();
	println!("Server running at http://{}/", config.server_addr);
	server.await
}
