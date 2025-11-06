use serde::de::DeserializeOwned;
use surrealdb::{Error, Response};

use crate::{
    auth::services::entities::user::NewUser,
    shared::{etities::userdb::User, repository::db::DB},
};

fn parse_error(error: Error) -> (u16, String) {
    if error.to_string().contains("username") {
        return (409, "Username already exists".to_string());
    }

    (500, format!("DB register error: {error}"))
}

async fn respose_parse<T: ToString + DeserializeOwned>(
    res: Result<Response, Error>,
) -> Result<String, (u16, String)> {
    let mut res = match res {
        Ok(res) => res,
        Err(e) => return Err((500, format!("DB query error: {e}"))),
    };

    let data: Option<T> = match res.take(0) {
        Ok(data) => data,
        Err(e) => return Err(parse_error(e)),
    };

    match data {
        Some(data) => Ok(data.to_string()),
        None => Err((500, "DB parse error".to_string())),
    }
}

pub async fn create_user(user: NewUser) -> Result<String, (u16, String)> {
    let query = r#"
    CREATE user content {
        name: $name,
        lastname: $lastname,
        username: $username,
        password: $password
    }
    "#;

    let result = DB
        .query(query)
        .bind(("username", user.username()))
        .bind(("name", user.name()))
        .bind(("lastname", user.lastname()))
        .bind(("password", user.password()))
        .await;

    respose_parse::<User>(result).await
}
