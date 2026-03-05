use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Income {
    pub id: String,
    pub amount: f64,
    pub description: String,
    pub processed: bool,
    pub date: String,
}
