use crate::routes::{health_check, subscribe};
use actix_web::{HttpServer, web, App};
use actix_web::dev::Server;
use sqlx::PgPool;
use std::net::TcpListener;
use std::io;

pub fn run(listener: TcpListener,
	   db_pool: PgPool
	) -> Result<Server, io::Error> {
		let db_pool = web::Data::new(db_pool);
		//capture connection from the surrounding environment
		let server = HttpServer::new(move || {
			App::new()
			.route("/health_check", web::get().to(health_check))
			.route("/subscriptions", web::post().to(subscribe))
			//Register the connection as part of app state
			.app_data(db_pool.clone())
		})
			.listen(listener)?
			.run();
		
		Ok(server)

    }