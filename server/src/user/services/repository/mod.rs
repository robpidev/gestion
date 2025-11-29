mod username;
pub use username::UsernameRepository;

use crate::shared::etities::userdb::{ToUser, User};
use crate::{shared::repository::db::DB, user::services::entities::UserUpdate};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Deserialize)]
pub struct UserDB {
    id: Thing,
    name: String,
    lastname: String,
    username: String,
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

pub struct NameLastNameUpdateRepository {
    query: String,
}

impl NameLastNameUpdateRepository {
    pub fn new() -> Self {
        Self {
            query: r#"
            UPDATE type::thing("user", $id) SET
            "#
            .to_string(),
        }
    }

    fn make_query(&mut self, user: &UserUpdate) {
        if let Some(_name) = &user.get_name() {
            self.query.push_str("name = $name");
        };

        if let Some(_lastname) = &user.get_lastname() {
            self.query.push_str(", lastname = $lastname");
        };
    }

    pub async fn update_user(
        &mut self,
        user: UserUpdate,
    ) -> Result<impl Serialize + use<>, (u16, String)> {
        self.make_query(&user);
        let mut resp = match DB
            .query(&self.query)
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
            Some(u) => Ok(u.to_user()),
            None => return Err((404, "User not found".to_string())),
        }
    }
}
