mod services;
mod signin;
mod signup;
use actix_web::{HttpResponse, Responder, get, web};

#[get("")]
async fn auth() -> impl Responder {
    HttpResponse::Ok().body("Auth")
}

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .service(auth)
            .configure(signup::routes)
            .configure(signin::routes),
    );
}
