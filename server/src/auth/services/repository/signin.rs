use crate::shared::etities::userdb::{ToUser, User};
use crate::shared::repository::db::DB;
use serde::Deserialize;
use surrealdb::sql::Thing;

#[derive(Deserialize)]
struct UserDB {
    id: Thing,
    username: String,
    name: String,
    lastname: String,
}

impl ToUser for UserDB {
    fn to_user(&self) -> User {
        User {
            id: self.id.id.to_string(),
            username: self.username.clone(),
            name: self.name.clone(),
            lastname: self.lastname.clone(),
        }
    }
}

pub struct SigninRepository {
    query: &'static str,
}

impl SigninRepository {
    pub fn new() -> SigninRepository {
        SigninRepository {
            query: r#"
            SELECT * FROM user
            WHERE username = $username AND password = $password;
            "#,
        }
    }

    pub async fn signin(self, username: &str, password: &str) -> Result<User, (u16, String)> {
        let res = DB
            .query(self.query)
            .bind(("username", username.to_string()))
            .bind(("password", password.to_string()))
            .await;

        let mut resp = match res {
            Ok(r) => r,
            Err(e) => return Err((500, format!("DB query error: {}", e))),
        };

        let user: Option<UserDB> = match resp.take(0) {
            Ok(user) => user,
            Err(e) => return Err((500, format!("DB error parsing user: {e}"))),
        };

        match user {
            Some(u) => Ok(u.to_user()),
            None => Err((404, "User or password invalid".to_string())),
        }
    }
}
