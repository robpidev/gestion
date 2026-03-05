mod entities;
mod repository;

use entities::Income;
use repository::IncomesRepository;
use serde::Serialize;
pub struct IncomesService;

pub use repository::NewIncome;

impl IncomesService {
    pub async fn get(user_id: String) -> Result<impl Serialize, (u16, String)> {
        IncomesRepository::get(user_id).await
    }

    pub async fn create(
        id: String,
        description: String,
        amount: f64,
        date: String,
        processed: bool,
    ) -> Result<impl Serialize, (u16, String)> {
        let new_income = NewIncome {
            description,
            amount,
            date,
            processed,
        };
        IncomesRepository::create(id, new_income).await
    }

    pub async fn delete(id: String, income_id: String) -> Result<impl Serialize, (u16, String)> {
        IncomesRepository::delete(id, income_id).await
    }

    pub async fn update(
        user_id: String,
        income_id: String,
        amount: f64,
        description: String,
        date: String,
        processed: bool,
    ) -> Result<impl Serialize, (u16, String)> {
        let new_income = NewIncome {
            description,
            amount,
            date,
            processed,
        };
        IncomesRepository::update(user_id, income_id, new_income).await
    }

    pub async fn update_amount(
        user_id: String,
        income_id: String,
        amount: f64,
    ) -> Result<impl Serialize, (u16, String)> {
        let key = "amount";
        IncomesRepository::update_field(user_id, income_id, key, amount).await
    }

    pub async fn update_description(
        user_id: String,
        income_id: String,
        description: String,
    ) -> Result<impl Serialize, (u16, String)> {
        let key = "description";
        IncomesRepository::update_field(user_id, income_id, key, description).await
    }

    pub async fn update_date(
        user_id: String,
        income_id: String,
        date: String,
    ) -> Result<impl Serialize, (u16, String)> {
        let key = "date";
        IncomesRepository::update_field(user_id, income_id, key, date).await
    }

    pub async fn update_processed(
        user_id: String,
        income_id: String,
        processed: bool,
    ) -> Result<impl Serialize, (u16, String)> {
        let key = "processed";
        IncomesRepository::update_field(user_id, income_id, key, processed).await
    }
}
