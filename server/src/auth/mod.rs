use actix_web::{HttpResponse, Responder, get, web};

#[get("/auth")]
async fn auth() -> impl Responder {
    HttpResponse::Ok().body("Auth")
}

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/auth").service(auth));
}
