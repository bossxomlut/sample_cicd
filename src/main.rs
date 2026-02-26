use actix_web::{get, App, HttpServer, Responder};
use dotenvy::dotenv;
use std::env;

#[get("/hello")]
async fn hello() -> impl Responder {
    let env_name = env::var("ENVIRONMENT_NAME").unwrap_or_else(|_| "dev".to_string());
    format!("Hello World 1999 v2\nBuild build Github Action\nENVIRONMENT_NAME={}", env_name)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load `.env` once at startup (if present)
    let _ = dotenv();

    HttpServer::new(|| {
        App::new()
            .service(hello)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
