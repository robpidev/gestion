use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Expense {
    pub id: String,
    pub amount: f32,
    pub description: String,
    pub processed: bool,
    pub date: String,
}
