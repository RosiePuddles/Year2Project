use actix_files::NamedFile;
use actix_web::{get, http::StatusCode, web, Either, HttpMessage, HttpRequest, HttpResponse, Responder, Route};

use crate::{logger::Logger, logger_wrap};

#[get("/")]
pub async fn index() -> impl Responder { NamedFile::open("front/index.html") }

#[get("/{file}.js")]
pub async fn js_files(file: web::Path<(String,)>) -> impl Responder {
	NamedFile::open(format!("front/{}.js", file.into_inner().0))
}

async fn not_found(req: HttpRequest) -> impl Responder {
	match NamedFile::open("front/404.html") {
		Ok(nf) => Either::Left(nf.customize().with_status(StatusCode::NOT_FOUND).respond_to(&req)),
		Err(e) => {
			let logger = req.app_data::<Logger>().expect("Logger not initialised");
			logger_wrap!(logger.error, req, format!("Unable to open 404 file (404.html) {:?}", e));
			Either::Right(HttpResponse::InternalServerError().respond_to(&req))
		}
	}
}

pub fn not_found_route() -> Route { Route::new().to(|req: HttpRequest| not_found(req)) }
