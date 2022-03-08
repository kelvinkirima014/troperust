use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use std::io;
use actix_web::dev::Server;
use std::net::TcpListener;

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

async fn subscribe() -> HttpResponse {
	HttpResponse::Ok().finish()
}

pub fn run(listener: TcpListener) -> Result<Server, io::Error> {
	let server = HttpServer::new(|| {
		App::new()
		.route("/health_check", web::get().to(health_check))
		.route("/subsriptions", web::post().to(subscribe))
		})
		.listen(listener)?
		.run();
	
	Ok(server)

    }