use actix_web::{HttpResponse, Responder, http::StatusCode, post, web};
use serde::{Deserialize, Serialize};

use crate::auth::services::SigninService;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/signin").service(signin));
}

#[derive(Deserialize, Serialize)]
struct Auth {
    username: String,
    password: String,
}

#[post("")]
async fn signin(auth: web::Form<Auth>) -> impl Responder {
    match SigninService::signin(&auth.username, &auth.password).await {
        Ok(data) => HttpResponse::Ok().json(data),
        Err((c, m)) => HttpResponse::build(StatusCode::from_u16(c).unwrap()).body(m),
    }
}
