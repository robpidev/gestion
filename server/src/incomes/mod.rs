use actix_web::web::ReqData;
use actix_web::{HttpResponse, Responder, http::StatusCode, post};
use actix_web::{delete, get, patch, web};
use serde::Deserialize;

mod services;

use crate::incomes::services::IncomesService;
use crate::shared::middlewares::UserJwt;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/income")
            .wrap(UserJwt)
            .service(incomes)
            .service(create)
            .service(delete)
            .service(update_all)
            .service(update_amount)
            .service(update_description)
            .service(update_date)
            .service(update_processed),
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

#[delete("/{income_id}")]
pub async fn delete(income_id: web::Path<String>, user_id: ReqData<String>) -> impl Responder {
    match IncomesService::delete(user_id.to_string(), income_id.to_string()).await {
        Ok(data) => HttpResponse::Ok().json(data),
        Err((c, m)) => HttpResponse::build(StatusCode::from_u16(c).unwrap()).body(m),
    }
}

// === Update amount ===
#[derive(Deserialize)]
struct UpdateAmount {
    amount: f64,
}

#[patch("/{income_id}/amount")]
pub async fn update_amount(
    path: web::Path<String>,
    amout: web::Form<UpdateAmount>,
    user_id: ReqData<String>,
) -> impl Responder {
    match IncomesService::update_amount(user_id.to_string(), path.to_string(), amout.amount).await {
        Ok(data) => HttpResponse::Ok().json(data),
        Err((c, m)) => HttpResponse::build(StatusCode::from_u16(c).unwrap()).body(m),
    }
}

// === Update description ===
#[derive(Deserialize)]
struct UpdateDescription {
    description: String,
}

#[patch("/{income_id}/description")]
pub async fn update_description(
    path: web::Path<String>,
    description: web::Form<UpdateDescription>,
    user_id: ReqData<String>,
) -> impl Responder {
    match IncomesService::update_description(
        user_id.to_string(),
        path.to_string(),
        description.description.to_string(),
    )
    .await
    {
        Ok(data) => HttpResponse::Ok().json(data),
        Err((c, m)) => HttpResponse::build(StatusCode::from_u16(c).unwrap()).body(m),
    }
}

// === Update date ===
#[derive(Deserialize)]
struct UpdateDate {
    date: String,
}

#[patch("/{income_id}/date")]
pub async fn update_date(
    path: web::Path<String>,
    date: web::Form<UpdateDate>,
    user_id: ReqData<String>,
) -> impl Responder {
    match IncomesService::update_date(user_id.to_string(), path.to_string(), date.date.to_string())
        .await
    {
        Ok(data) => HttpResponse::Ok().json(data),
        Err((c, m)) => HttpResponse::build(StatusCode::from_u16(c).unwrap()).body(m),
    }
}

// === Update processed ===
#[derive(Deserialize)]
struct UpdateProcessed {
    processed: bool,
}

#[patch("/{income_id}/processed")]
pub async fn update_processed(
    path: web::Path<String>,
    processed: web::Form<UpdateProcessed>,
    user_id: ReqData<String>,
) -> impl Responder {
    match IncomesService::update_processed(
        user_id.to_string(),
        path.to_string(),
        processed.processed,
    )
    .await
    {
        Ok(data) => HttpResponse::Ok().json(data),
        Err((c, m)) => HttpResponse::build(StatusCode::from_u16(c).unwrap()).body(m),
    }
}

// === Update All ===
#[patch("/{income_id}")]
pub async fn update_all(
    path: web::Path<String>,
    update: web::Form<NewIncome>,
    user_id: ReqData<String>,
) -> impl Responder {
    match IncomesService::update(
        user_id.to_string(),
        path.to_string(),
        update.amount,
        update.description.clone(),
        update.date.clone(),
        update.processed,
    )
    .await
    {
        Ok(data) => HttpResponse::Ok().json(data),
        Err((c, m)) => HttpResponse::build(StatusCode::from_u16(c).unwrap()).body(m),
    }
}
