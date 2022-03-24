use actix_web::{HttpResponse, web};
use sqlx::PgConnection;
use chrono::Utc;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
	email: String,
	name: String
}

pub async fn subscribe(
	form: web::Form<FormData>,
	//retrieving a connection from app state
	connection: web::Data<PgConnection>,
	) -> HttpResponse {
		sqlx::query!(
			r#"
			INSERT INTO subscriptions(

			id, email, name,subscribed_at	
			)
			VALUES ($1, $2, $3, $4)
			"#,
			Uuid::new_v4(),
			form.email,
			form.name,
			Utc::now()
		)
		//`get_ref` gets an immutable reference to PgConnection
		.execute(connection.get_ref()).await;
		HttpResponse::Ok().finish()
}