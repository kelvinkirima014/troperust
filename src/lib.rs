use actix_web::{web, App, HttpServer, HttpResponse};

async fn get_response() -> HttpResponse {
	HttpResponse::Ok().finish()
}

pub async fn run() -> std::io::Result<()> {
	HttpServer::new(|| {
		App::new()
		.route("/get_response", web::get().to(get_response))
	})
	.bind("127.0.0.1:8000").unwrap()
	.run()
	.await
}