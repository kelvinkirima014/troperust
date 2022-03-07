use actix_web::{web, App, HttpRequest, HttpServer, HttpResponse, Responder};
use std::io;
use actix_web::dev::Server;
use std::net::TcpListener;

async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

pub fn run(listener: TcpListener) -> Result<Server, io::Error> {
	let server = HttpServer::new(|| {
		App::new()
		.route("/health_check", web::get().to(health_check))
		})
		.listen(listener)?
		.run();
	
	Ok(server)

    }