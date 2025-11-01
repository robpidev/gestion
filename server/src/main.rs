mod config;

use actix_web::{self, App, HttpResponse, HttpServer, Responder, get};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world w")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server is starting...");

    let config = match config::Config::from_env() {
        Ok(config) => {
            println!("Server is starting on {}:{}", config.host, config.port);
            config
        }
        Err(e) => {
            println!("Failed to load config: {}", e);
            std::process::exit(1);
        }
    };

    HttpServer::new(|| App::new().service(hello))
        .bind((config.host, config.port))?
        .run()
        .await
}
