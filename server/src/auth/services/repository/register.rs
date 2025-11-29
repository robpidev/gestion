use serde::Deserialize;
use serde::de::DeserializeOwned;
use surrealdb::sql::Thing;
use surrealdb::{Error, Response};

use crate::{
    auth::services::entities::user::NewUser,
    shared::{etities::userdb::User, repository::db::DB},
};

#[derive(Deserialize)]
struct UserDB {
    id: Thing,
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
            id: self.id.id.to_string(),
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

    pub async fn register_user(&self, user: NewUser) -> Result<User, (u16, String)> {
        let result = DB
            .query(self.query)
            .bind(("username", user.username()))
            .bind(("name", user.name()))
            .bind(("lastname", user.lastname()))
            .bind(("password", user.password()))
            .await;

        Self::respose_parse::<UserDB>(result).await
    }

    fn parse_error(error: impl ToString + std::fmt::Display) -> (u16, String) {
        if error.to_string().contains("username") {
            return (409, "Username already exists".to_string());
        }

        (500, format!("DB register error: {error}"))
    }

    async fn respose_parse<T: ToUser + DeserializeOwned>(
        res: Result<Response, Error>,
    ) -> Result<User, (u16, String)> {
        let mut res = match res {
            Ok(res) => res,
            Err(e) => return Err((500, format!("DB query error: {e}"))),
        };

        let data: Option<T> = match res.take(0) {
            Ok(data) => data,
            Err(e) => return Err(Self::parse_error(e)),
        };

        match data {
            Some(data) => Ok(data.to_user()),
            None => Err((500, "DB parse error".to_string())),
        }
    }
}
