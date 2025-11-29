use actix_web::web::ReqData;
use actix_web::{HttpResponse, Responder, get, http::StatusCode, patch, web};
use serde::{Deserialize, Serialize};

mod services;
use services::UserUpdateService;
use services::UsernameService;

use crate::shared::middlewares::UserJwt;

#[get("")]
async fn user() -> impl Responder {
    HttpResponse::Ok().body("User")
}

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/user")
            .wrap(UserJwt)
            .service(user)
            .service(user_update)
            .service(username_update)
            .service(username_exists),
    );
}

//=== user update data ===

#[derive(Deserialize, Serialize)]
struct UserUpdateData {
    name: Option<String>,
    lastname: Option<String>,
}

fn responder_parse(data: Result<impl Serialize, (u16, String)>) -> impl Responder {
    match data {
        Ok(data) => HttpResponse::Ok().json(data),
        Err((c, m)) => HttpResponse::build(StatusCode::from_u16(c).unwrap()).body(m),
    }
}

#[patch("/names")]
async fn user_update(id: ReqData<String>, data: web::Form<UserUpdateData>) -> impl Responder {
    match UserUpdateService::update_user(id.to_string(), data.name.clone(), data.lastname.clone())
        .await
    {
        Ok(data) => HttpResponse::Ok().json(data),
        Err((c, m)) => HttpResponse::build(StatusCode::from_u16(c).unwrap()).body(m),
    }
}

#[get("/username/{username}")]
async fn username_exists(username: web::Path<String>) -> impl Responder {
    responder_parse(UsernameService::exists(username.to_string()).await)
}

#[derive(Deserialize)]
struct UsernameUpdateData {
    username: String,
}

#[patch("/username")]
async fn username_update(
    id: ReqData<String>,
    username: web::Form<UsernameUpdateData>,
) -> impl Responder {
    // return HttpResponse::Ok().json(username.username.clone());
    responder_parse(
        UsernameService::update_username(id.to_string(), username.username.clone()).await,
    )
}
