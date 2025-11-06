use crate::shared::etities::userdb::User;
use crate::shared::repository::db::DB;

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

    let mut user_opt = match res {
        Ok(user) => user,
        Err(e) => return Err((500, format!("DB query error: {}", e))),
    };

    let user: Option<User> = match user_opt.take(0) {
        Ok(user) => user,
        Err(e) => return Err((500, format!("DB error parsing user: {e}"))),
    };

    match user {
        Some(user) => Ok(user),
        None => Err((404, "User not found".to_string())),
    }
}
