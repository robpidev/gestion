use actix_web::web::ReqData;
use actix_web::{HttpResponse, Responder, http::StatusCode, post};
use actix_web::{get, web};
use serde::Deserialize;

mod services;

use crate::incomes::services::IncomesService;
use crate::shared::middlewares::UserJwt;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/incomes")
            .wrap(UserJwt)
            .service(incomes)
            .service(create),
    );
}

#[get("")]
pub async fn incomes(user_id: ReqData<String>) -> impl Responder {
    match IncomesService::get(user_id.to_string()).await {
        Ok(data) => HttpResponse::Ok().json(data),
        Err((c, m)) => HttpResponse::build(StatusCode::from_u16(c).unwrap()).body(m),
    }
}

#[derive(Deserialize)]
struct NewIncome {
    amount: f64,
    description: String,
    processed: bool,
    date: String,
}

#[post("")]
pub async fn create(new_income: web::Form<NewIncome>, id: ReqData<String>) -> impl Responder {
    match IncomesService::create(
        id.to_string(),
        new_income.description.clone(),
        new_income.amount,
        new_income.date.clone(),
        new_income.processed,
    )
    .await
    {
        Ok(data) => HttpResponse::Ok().json(data),
        Err((c, m)) => HttpResponse::build(StatusCode::from_u16(c).unwrap()).body(m),
    }
}
