use serde::Deserialize;
// use serde::de::DeserializeOwned;
// use surrealdb::sql::Thing;
// use surrealdb::Error;
use surrealdb_types::{RecordId, RecordIdKey, SurrealValue};

use crate::{
    auth::services::entities::user::NewUser,
    shared::{etities::userdb::User, repository::db::DB},
};

#[derive(Debug, Deserialize, SurrealValue)]
struct UserDB {
    id: RecordId,
    name: String,
    lastname: String,
    username: String,
}

trait ToUser {
    fn to_user(&self) -> User;
}

impl ToUser for UserDB {
    fn to_user(&self) -> User {
        User {
            id: match &self.id.key {
                RecordIdKey::String(id) => id.clone(),
                _ => "".to_string(),
            },
            name: self.name.to_string(),
            lastname: self.lastname.to_string(),
            username: self.username.to_string(),
        }
    }
}

pub struct RegisterRepository {
    query: &'static str,
}

impl RegisterRepository {
    pub fn new() -> Self {
        Self {
            query: r#"
            CREATE user content {
                name: $name,
                lastname: $lastname,
                username: $username,
                password: $password
            }
            "#,
        }
    }

    pub async fn create_user(&self, user: NewUser) -> Result<User, (u16, String)> {
        let result = DB
            .query(self.query)
            .bind(("username", user.username()))
            .bind(("name", user.name()))
            .bind(("lastname", user.lastname()))
            .bind(("password", user.password()))
            .await;

        let mut resp = match result {
            Ok(res) => res,
            Err(e) => return Err((500, format!("DB query error: {e}"))),
        };

        let data: Option<UserDB> = match resp.take(0) {
            Ok(data) => data,
            Err(e) => return Err(Self::parse_error(e)),
        };

        match data {
            Some(data) => Ok(data.to_user()),
            None => Err((500, "DB parse error".to_string())),
        }
    }

    fn parse_error(error: impl ToString + std::fmt::Display) -> (u16, String) {
        // TODO: parse error RETURN Connection uninitialised
        if error.to_string().contains("index_username") {
            return (409, "Username already exists".to_string());
        }

        (500, format!("DB register error: {error}"))
    }
}
