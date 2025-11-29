use serde::Serialize;

use crate::{shared::etities::username::Username, user::services::repository::UsernameRepository};

pub struct UsernameService;

impl UsernameService {
    pub async fn exists(username: String) -> Result<bool, (u16, String)> {
        UsernameRepository::exists(username).await
    }

    pub async fn update_username(
        id: String,
        username: String,
    ) -> Result<impl Serialize, (u16, String)> {
        let username = match Username::new(&username) {
            Ok(u) => u,
            Err(e) => return Err((400, e.to_string())),
        };
        UsernameRepository::update_username(id, username.to_string()).await
    }
}
