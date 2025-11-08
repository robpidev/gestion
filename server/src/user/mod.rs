mod services;
use actix_web::{HttpResponse, Responder, get, http::StatusCode, patch, web};
use serde::{Deserialize, Serialize};

#[get("")]
async fn user() -> impl Responder {
    HttpResponse::Ok().body("User")
}

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/user").service(user).service(user_update));
}

//=== user update data ===

#[derive(Deserialize, Serialize)]
struct UserUpdateData {
    name: Option<String>,
    lastname: Option<String>,
}

#[patch("/{id}")]
async fn user_update(id: web::Path<String>, data: web::Form<UserUpdateData>) -> impl Responder {
    match services::update_user(id.to_string(), data.name.clone(), data.lastname.clone()).await {
        Ok(data) => HttpResponse::Ok().json(data),
        Err((c, m)) => HttpResponse::build(StatusCode::from_u16(c).unwrap()).body(m),
    }
}
