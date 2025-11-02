use actix_web::{HttpResponse, Responder, get, web};

#[get("")]
async fn user() -> impl Responder {
    HttpResponse::Ok().body("User")
}

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/user").service(user));
}
