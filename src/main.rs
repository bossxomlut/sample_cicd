use actix_web::{get, App, HttpServer, Responder};
use dotenvy::dotenv;
use std::env;

#[get("/hello")]
async fn hello() -> impl Responder {
     // Load `.env` if present (created by CI)
    let _ = dotenv();

    let env_name = env::var("ENVIRONMENT_NAME").unwrap_or_else(|_| "dev".to_string());
    println!("ENVIRONMENT_NAME={}", env_name);

    format!("Hello World 1999 v2\nBuild build Github Action\nENVIRONMENT_NAME={}", env_name) 
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
