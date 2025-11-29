use actix_web::{HttpResponse, Responder, http::StatusCode, post, web};

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/signup").service(signup));
}

use serde::{Deserialize, Serialize};

use super::services;

#[derive(Deserialize, Serialize)]
struct NewUser {
    name: String,
    lastname: String,
    username: String,
    password: String,
}

#[post("")]
async fn signup(user: web::Form<NewUser>) -> impl Responder {
    let data =
        services::signup::register(&user.name, &user.lastname, &user.username, &user.password)
            .await;
    match data {
        Ok(data) => HttpResponse::Ok().json(data),
        Err((code, msg)) => HttpResponse::build(StatusCode::from_u16(code).unwrap()).body(msg),
    }
}
