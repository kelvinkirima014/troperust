use actix_web::{HttpResponse, web};
use sqlx::PgPool;
use chrono::Utc;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
	email: String,
	name: String
}

pub async fn subscribe() {
	todo!()
}