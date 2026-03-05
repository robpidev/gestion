mod entities;
mod repository;

use entities::Expense;
use repository::ExpensesRepository;
use serde::Serialize;
pub struct ExpensesService;

pub use repository::NewExpense;

impl ExpensesService {
    pub async fn get(user_id: String) -> Result<impl Serialize, (u16, String)> {
        ExpensesRepository::get(user_id).await
    }

    pub async fn create(
        id: String,
        description: String,
        amount: f64,
        date: String,
        processed: bool,
    ) -> Result<impl Serialize, (u16, String)> {
        let new_expense = NewExpense {
            description,
            amount,
            date,
            processed,
        };
        ExpensesRepository::create(id, new_expense).await
    }

    pub async fn delete(id: String, expense_id: String) -> Result<impl Serialize, (u16, String)> {
        ExpensesRepository::delete(id, expense_id).await
    }

    pub async fn update(
        user_id: String,
        expense_id: String,
        amount: f64,
        description: String,
        date: String,
        processed: bool,
    ) -> Result<impl Serialize, (u16, String)> {
        let new_expense = NewExpense {
            description,
            amount,
            date,
            processed,
        };
        ExpensesRepository::update(user_id, expense_id, new_expense).await
    }

    pub async fn update_amount(
        user_id: String,
        expense_id: String,
        amount: f64,
    ) -> Result<impl Serialize, (u16, String)> {
        let key = "amount";
        ExpensesRepository::update_field(user_id, expense_id, key, amount).await
    }

    pub async fn update_description(
        user_id: String,
        expense_id: String,
        description: String,
    ) -> Result<impl Serialize, (u16, String)> {
        let key = "description";
        ExpensesRepository::update_field(user_id, expense_id, key, description).await
    }

    pub async fn update_date(
        user_id: String,
        expense_id: String,
        date: String,
    ) -> Result<impl Serialize, (u16, String)> {
        let key = "date";
        ExpensesRepository::update_field(user_id, expense_id, key, date).await
    }

    pub async fn update_processed(
        user_id: String,
        expense_id: String,
        processed: bool,
    ) -> Result<impl Serialize, (u16, String)> {
        let key = "processed";
        ExpensesRepository::update_field(user_id, expense_id, key, processed).await
    }
}
