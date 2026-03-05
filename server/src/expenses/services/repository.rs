use serde::Deserialize;
use surrealdb::IndexedResults;
use surrealdb_types::Datetime;
use surrealdb_types::RecordId;
use surrealdb_types::RecordIdKey;
use surrealdb_types::SurrealValue;

use crate::shared::repository::DB;

use super::Expense;

#[derive(Debug, SurrealValue, Deserialize)]
pub struct NewExpense {
    pub amount: f64,
    pub description: String,
    pub processed: bool,
    pub date: String,
}

#[derive(Debug, SurrealValue)]
struct ExpenseDB {
    id: RecordId,
    amount: f64,
    description: String,
    processed: bool,
    date: Datetime,
}

impl From<ExpenseDB> for Expense {
    fn from(value: ExpenseDB) -> Self {
        return Expense {
            id: match &value.id.key {
                RecordIdKey::String(id) => id.clone(),
                _ => "".to_string(),
            },
            amount: value.amount,
            description: value.description,
            processed: value.processed,
            date: value.date.to_string(),
        };
    }
}

pub struct ExpensesRepository;

impl ExpensesRepository {
    pub async fn get(user_id: String) -> Result<Vec<Expense>, (u16, String)> {
        let query = r#"
select * from type::record("user", $user_id)->had->expense
            "#;

        let mut result = match DB.query(query).bind(("user_id", user_id)).await {
            Ok(result) => result,
            Err(e) => return Err((500, e.to_string())),
        };

        let resp: Vec<ExpenseDB> = match result.take(0) {
            Ok(result) => result,
            Err(e) => return Err((500, e.to_string())),
        };

        return Ok(resp.into_iter().map(|i| i.into()).collect());
    }

    pub async fn create(
        user_id: String,
        new_expense: NewExpense,
    ) -> Result<Expense, (u16, String)> {
        let query = r#"
BEGIN TRANSACTION;
LET $expense = (CREATE expense content {
    description: $description,
    amount: $amount,
    date: <datetime>$date,
    processed: $processed
});
LET $user = type::record("user", $user_id);
RELATE $user->had->$expense;
return $expense;
COMMIT TRANSACTION;
            "#;

        let result = match DB
            .query(query)
            .bind(("user_id", user_id))
            .bind(("description", new_expense.description.clone()))
            .bind(("date", new_expense.date + "T00:00:00Z"))
            .bind(("amount", new_expense.amount))
            .bind(("processed", new_expense.processed))
            .await
        {
            Ok(result) => result,
            Err(e) => return Err((500, e.to_string())),
        };
        dbg!(&result);
        Self::response_expense_parse(result, 4)
    }

    pub async fn delete(user_id: String, expense_id: String) -> Result<Expense, (u16, String)> {
        let query = r#"
LET $user = type::record("user",$user_id);
LET $expense = type::record("expense", $expense_id);
LET $relation = select * from $user->had where out = $expense;
IF $relation {
    delete $expense RETURN before;
} ELSE {
    THROW "Expense for user not found"
}            "#;

        let result = match DB
            .query(query)
            .bind(("user_id", user_id))
            .bind(("expense_id", expense_id))
            .await
        {
            Ok(result) => result,
            Err(e) => return Err((500, format!("DB Query error: {}", e.to_string()))),
        };

        Self::response_expense_parse(result, 3)
    }

    pub async fn update(
        user_id: String,
        expense_id: String,
        new_expense: NewExpense,
    ) -> Result<Expense, (u16, String)> {
        let query = r#"
            LET $user = type::record("user", $user_id);
LET $expense = type::record("expense", $expense_id);
LET $relation = select * from $user->had where out = $expense;
IF $relation {{
    UPDATE $expense set description = $description, amount = $amount, date = <datetime>$date, processed = $processed return after
}} else {{
    THROW "Expense not found for user"
}}
            "#;

        let result = match DB
            .query(query)
            .bind(("user_id", user_id))
            .bind(("description", new_expense.description.clone()))
            .bind(("date", new_expense.date + "T00:00:00Z"))
            .bind(("amount", new_expense.amount))
            .bind(("processed", new_expense.processed))
            .bind(("expense_id", expense_id))
            .await
        {
            Ok(result) => result,
            Err(e) => return Err((500, format!("DB Query error: {}", e.to_string()))),
        };

        Self::response_expense_parse(result, 3)
    }

    pub async fn update_field<T: Clone + SurrealValue>(
        user_id: String,
        expense_id: String,
        key: &'static str,
        value: T,
    ) -> Result<Expense, (u16, String)> {
        let query = format!(
            r#"
            LET $user = type::record("user", $user_id);
LET $expense = type::record("expense", $expense_id);
LET $relation = select * from $user->had where out = $expense;
IF $relation {{
    UPDATE $expense set {} = $value return after
}} else {{
    THROW "Expense not found for user"
}}
            "#,
            key
        );

        let result = match DB
            .query(query)
            .bind(("user_id", user_id))
            .bind(("value", value))
            .bind(("expense_id", expense_id))
            .await
        {
            Ok(result) => result,
            Err(e) => return Err((500, format!("DB Query error: {}", e.to_string()))),
        };

        Self::response_expense_parse(result, 3)
    }

    fn response_expense_parse(
        result: IndexedResults,
        index: usize,
    ) -> Result<Expense, (u16, String)> {
        let mut idex_results = match result.check() {
            Ok(r) => r,
            Err(e) => return Err(Self::error_parse(e.to_string())),
        };

        dbg!(&idex_results);

        let resp: Option<ExpenseDB> = match idex_results.take(index) {
            Ok(resp) => resp,
            Err(e) => {
                dbg!(&e);
                return Err((500, format!("DB Error Parse {}", e.message())));
            }
        };

        match resp {
            Some(expense) => return Ok(expense.into()),
            None => return Err((500, "DB Take return NONE".to_string())),
        }
    }

    pub fn error_parse(e: String) -> (u16, String) {
        let message = "Expense for user not found".to_string();
        if e.contains(&message) {
            return (400, message);
        } else {
            return (500, format!("DB Response error: {}", e));
        }
    }
}
