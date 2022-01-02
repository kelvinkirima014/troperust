use actix_web::{web, App, HttpResponse, HttpServer};
async fn health_check() -> HttpResponse{
	todo!()
}
pub async fn run() -> std::io::Result<()> {
	HttpServer::new(|| {
		App::new() 
		.route("/health_check", web::get()
		.to(health_check))
	})
	.bind("127.0.0:8000")?
	.run()
	.await
}