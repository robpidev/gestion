use crate::shared::etities::userdb::User;
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

pub async fn signin(username: &str, password: &str) -> Result<User, (u16, String)> {
    let query = r#"
    SELECT * FROM user
    WHERE username = $username AND password = $password;
    "#;

    let res = DB
        .query(query)
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
        Some(user) => Ok({
            User {
                id: user.id.id.to_string(),
                username: user.username,
                name: user.name,
                lastname: user.lastname,
            }
        }),
        None => Err((404, "User or password invalid".to_string())),
    }
}
