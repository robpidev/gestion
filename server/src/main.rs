use server::auth;
use server::config;
use server::config::server::Server;
use server::shared::repository::db;

use actix_web::{self, App, HttpResponse, HttpServer, Responder, get};
use server::user;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server is starting...");

    // Load .env file
    config::load_env();

    // Config server
    let server_config = Server::from_env();

    println!("Host: {}", server_config.host);
    println!("Port: {}", server_config.port);

    // config db
    if let Err(e) = db::db_connect().await {
        panic!("{e}");
    }

    HttpServer::new(|| {
        App::new()
            .service(hello)
            .configure(auth::routes)
            .configure(user::routes)
    })
    .bind((server_config.host, server_config.port))?
    .run()
    .await
}
