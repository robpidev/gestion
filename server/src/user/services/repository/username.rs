use crate::shared::{etities::userdb::ToUser, repository::DB};
use serde::{Deserialize, Serialize};

use super::UserDB;

pub struct UsernameRepository;

#[derive(Deserialize)]
#[allow(dead_code)]
struct UsernameDB {
    username: String,
}

impl UsernameRepository {
    pub async fn exists(username: String) -> Result<bool, (u16, String)> {
        let query = r"
        SELECT username FROM user
        WHERE username = $username;
        ";

        let resp = DB.query(query).bind(("username", username)).await;

        let mut opt = match resp {
            Ok(r) => r,
            Err(e) => return Err((500, format!("DB error: {}", e))),
        };

        let user: Option<UsernameDB> = match opt.take(0) {
            Ok(user) => user,
            Err(e) => return Err((500, format!("DB error parsing user: {e}"))),
        };

        match user {
            Some(_) => Ok(true),
            None => Ok(false),
        }
    }

    pub async fn update_username(
        id: String,
        username: String,
    ) -> Result<impl Serialize, (u16, String)> {
        let query = r#"
        UPDATE type::thing("user", $id)
        SET username = $username
        "#;

        let mut resp = match DB
            .query(query)
            .bind(("id", id))
            .bind(("username", username))
            .await
        {
            Ok(r) => r,
            Err(e) => return Err((500, format!("DB error: {}", e))),
        };

        let opt: Option<UserDB> = match resp.take(0) {
            Ok(user) => user,
            Err(e) => return Err((500, format!("DB error parsing user: {e}"))),
        };

        match opt {
            Some(u) => Ok(u.to_user()),
            None => Err((401, "Username not updated".to_string())),
        }
    }
}
