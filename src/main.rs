use actix_web::{web, HttpRequest, Responder, App, HttpServer};
use std::io;
async fn greet(req: HttpRequest) -> impl Responder{
	let name = req.match_info().get("name").unwrap_or("World");
	format!("Hello {}!", name)
}
#[actix_web::main]
async fn main() -> io::Result<()> {
	HttpServer::new(|| {
		App::new()
		.route("/", web::get().to(greet))
		.route("/{name}", web::get().to(greet))
	})
	.bind("127.0.0.1:8000")?
	.run()
	.await
}