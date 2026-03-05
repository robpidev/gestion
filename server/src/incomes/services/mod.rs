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
}
