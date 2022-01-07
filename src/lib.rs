use actix_web::{web, App, HttpServer, HttpResponse};
use actix_web::dev::Server;

async fn get_response() -> HttpResponse {
	HttpResponse::Ok().finish()
}

pub fn run() -> Result<Server, std::io::Error> {
	let server = HttpServer::new(|| {
		App::new()
		.route("/get_response", web::get().to(get_response))
	})
	.bind("127.0.0.1:8000")?
	.run();
	Ok(server)
}