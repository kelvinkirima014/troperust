use actix_web::{web, App, HttpServer, HttpResponse};
use actix_web::dev::Server;
use std::net::TcpListener;

async fn get_response() -> HttpResponse {
	HttpResponse::Ok().finish()
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
	let server = HttpServer::new(|| {
		App::new()
		.route("/get_response", web::get().to(get_response))
	})
	.listen(listener)?
	.run();
	Ok(server)
}