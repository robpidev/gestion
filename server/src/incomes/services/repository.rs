use serde::Deserialize;
use surrealdb::IndexedResults;
use surrealdb_types::Datetime;
use surrealdb_types::RecordId;
use surrealdb_types::RecordIdKey;
use surrealdb_types::SurrealValue;

use crate::shared::repository::DB;

use super::Income;

#[derive(Debug, SurrealValue, Deserialize)]
pub struct NewIncome {
    pub amount: f64,
    pub description: String,
    pub processed: bool,
    pub date: String,
}

#[derive(Debug, SurrealValue)]
struct IncomeDB {
    id: RecordId,
    amount: f64,
    description: String,
    processed: bool,
    date: Datetime,
}

impl From<IncomeDB> for Income {
    fn from(value: IncomeDB) -> Self {
        return Income {
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

pub struct IncomesRepository;

impl IncomesRepository {
    pub async fn get(user_id: String) -> Result<Vec<Income>, (u16, String)> {
        let query = r#"
select * from type::record("user", $user_id)->had->income
            "#;

        let mut result = match DB.query(query).bind(("user_id", user_id)).await {
            Ok(result) => result,
            Err(e) => return Err((500, e.to_string())),
        };

        let resp: Vec<IncomeDB> = match result.take(0) {
            Ok(result) => result,
            Err(e) => return Err((500, e.to_string())),
        };

        return Ok(resp.into_iter().map(|i| i.into()).collect());
    }

    pub async fn create(user_id: String, new_income: NewIncome) -> Result<Income, (u16, String)> {
        let query = r#"
BEGIN TRANSACTION;
LET $income = (CREATE income content {
    description: $description,
    amount: $amount,
    date: <datetime>$date,
    processed: $processed
});
LET $user = type::record("user", $user_id);
RELATE $user->had->$income;
return $income;
COMMIT TRANSACTION;
            "#;

        let mut result = match DB
            .query(query)
            .bind(("user_id", user_id))
            .bind(("description", new_income.description.clone()))
            .bind(("date", new_income.date + "T00:00:00Z"))
            .bind(("amount", new_income.amount))
            .bind(("processed", new_income.processed))
            .await
        {
            Ok(result) => result,
            Err(e) => return Err((500, e.to_string())),
        };

        // dbg!(&result);

        let resp: Option<IncomeDB> = match result.take(4) {
            Ok(result) => result,
            Err(e) => return Err((500, e.to_string())),
        };

        match resp {
            Some(income) => return Ok(income.into()),
            None => return Err((500, "Parse error creating income".to_string())),
        }
    }

    pub async fn delete(user_id: String, income_id: String) -> Result<Income, (u16, String)> {
        let query = r#"
LET $user = type::record("user",$user_id);
LET $income = type::record("income", $income_id);
LET $relation = select * from $user->had where out = $income;
IF $relation {
    delete $income RETURN before;
} ELSE {
    THROW "Income for user not found"
}            "#;

        let result = match DB
            .query(query)
            .bind(("user_id", user_id))
            .bind(("income_id", income_id))
            .await
        {
            Ok(result) => result,
            Err(e) => return Err((500, format!("DB Query error: {}", e.to_string()))),
        };

        Self::response_income_parse(result)
    }

    pub async fn update(
        user_id: String,
        income_id: String,
        new_income: NewIncome,
    ) -> Result<Income, (u16, String)> {
        let query = r#"
            LET $user = type::record("user", $user_id);
LET $income = type::record("income", $income_id);
LET $relation = select * from $user->had where out = $income;
IF $relation {{
    UPDATE $income set description = $description, amount = $amount, date = <datetime>$date, processed = $processed return after
}} else {{
    THROW "Income not found for user"
}}
            "#;

        let result = match DB
            .query(query)
            .bind(("user_id", user_id))
            .bind(("description", new_income.description.clone()))
            .bind(("date", new_income.date + "T00:00:00Z"))
            .bind(("amount", new_income.amount))
            .bind(("processed", new_income.processed))
            .bind(("income_id", income_id))
            .await
        {
            Ok(result) => result,
            Err(e) => return Err((500, format!("DB Query error: {}", e.to_string()))),
        };

        Self::response_income_parse(result)
    }

    pub async fn update_field<T: Clone + SurrealValue>(
        user_id: String,
        income_id: String,
        key: &'static str,
        value: T,
    ) -> Result<Income, (u16, String)> {
        let query = format!(
            r#"
            LET $user = type::record("user", $user_id);
LET $income = type::record("income", $income_id);
LET $relation = select * from $user->had where out = $income;
IF $relation {{
    UPDATE $income set {} = $value return after
}} else {{
    THROW "Income not found for user"
}}
            "#,
            key
        );

        let result = match DB
            .query(query)
            .bind(("user_id", user_id))
            .bind(("value", value))
            .bind(("income_id", income_id))
            .await
        {
            Ok(result) => result,
            Err(e) => return Err((500, format!("DB Query error: {}", e.to_string()))),
        };

        Self::response_income_parse(result)
    }

    fn response_income_parse(result: IndexedResults) -> Result<Income, (u16, String)> {
        let mut idex_results = match result.check() {
            Ok(r) => r,
            Err(e) => return Err(Self::error_parse(e.to_string())),
        };

        let resp: Option<IncomeDB> = match idex_results.take(3) {
            Ok(resp) => resp,
            Err(e) => {
                dbg!(&e);
                return Err((500, format!("DB Error Parse {}", e.message())));
            }
        };

        match resp {
            Some(income) => return Ok(income.into()),
            None => return Err((500, "DB Take return NONE".to_string())),
        }
    }

    pub fn error_parse(e: String) -> (u16, String) {
        let message = "Income for user not found".to_string();
        if e.contains(&message) {
            return (400, message);
        } else {
            return (500, format!("DB Response error: {}", e));
        }
    }
}
