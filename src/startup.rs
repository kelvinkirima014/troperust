use crate::routes::{health_check, subscribe};
use actix_web::{HttpServer, web, App};
use actix_web::dev::Server;
use std::net::TcpListener;
use std::io;

pub fn run(listener: TcpListener) -> Result<Server, io::Error> {
	let server = HttpServer::new(|| {
		App::new()
		.route("/health_check", web::get().to(health_check))
		.route("/subscriptions", web::post().to(subscribe))
		})
		.listen(listener)?
		.run();
	
	Ok(server)

    }