use actix_web::{Response, HttpRequest};
async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

use troperust::run;
#[actix_web::main]
async fn main() -> std::io:: Result<()> {
    run().await
    }


