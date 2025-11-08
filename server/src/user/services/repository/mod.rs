use crate::shared::etities::userdb::User;
use crate::{shared::repository::db::DB, user::services::entities::UserUpdate};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Deserialize)]
struct UserDB {
    id: Thing,
    name: String,
    lastname: String,
    username: String,
}

pub async fn update_user(user: UserUpdate) -> Result<impl Serialize, (u16, String)> {
    let mut query = r#"
    UPDATE type::thing("user", $id) SET
    "#
    .to_string();

    if let Some(_name) = &user.get_name() {
        query.push_str("name = $name");
    };

    if let Some(_lastname) = &user.get_lastname() {
        query.push_str(", lastname = $lastname");
    };
    query.push_str(" RETURN id, name, lastname, username");

    // TODO: username update should be unique
    let mut resp = match DB
        .query(&query)
        .bind(("id", user.id.to_string()))
        .bind(("name", user.get_name().unwrap_or("".to_string())))
        .bind(("lastname", user.get_lastname().unwrap_or("".to_string())))
        .await
    {
        Ok(r) => r,
        Err(e) => return Err((500, format!("DB query error: {e}"))),
    };

    let user_opt: Option<UserDB> = match resp.take(0) {
        Ok(u) => u,
        Err(e) => return Err((500, format!("DB parse error: {e}"))),
    };

    match user_opt {
        Some(u) => Ok(User {
            id: u.id.id.to_string(),
            name: u.name,
            lastname: u.lastname,
            username: u.username,
        }),
        None => return Err((404, "User not found".to_string())),
    }
}
