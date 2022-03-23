use actix_web::{HttpResponse, web};
use sqlx::PgConnection;

#[derive(serde::Deserialize)]
pub struct FormData {
	email: String,
	name: String
}

pub async fn subscribe(
	_form: web::Form<FormData>,
	//retrieving a connection from app state
	_connection: web::Data<PgConnection>
	) -> HttpResponse {
	HttpResponse::Ok().finish()
}