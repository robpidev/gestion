use actix_web::{HttpResponse, Responder, http::StatusCode, post, web};
use serde::{Deserialize, Serialize};

use crate::auth::services;

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
    match services::signin::signin(&auth.username, &auth.password).await {
        Ok(jwt) => HttpResponse::Ok().json(jwt),
        Err((c, m)) => HttpResponse::build(StatusCode::from_u16(c).unwrap()).body(m),
    }
}
