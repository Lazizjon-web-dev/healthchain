use actix_web::{get, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use std::{env, io};

#[actix_web::main]
async fn main() -> io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    HttpServer::new(move || App::new())
        .bind("0.0.0.0:8000")?
        .run()
        .await
}
