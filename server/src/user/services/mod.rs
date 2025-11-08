mod entities;
mod repository;
use entities::UserUpdate;
use serde::Serialize;

pub async fn update_user(
    id: String,
    name: Option<String>,
    lastname: Option<String>,
) -> Result<impl Serialize, (u16, String)> {
    if name.is_none() && lastname.is_none() {
        return Err((400, "All data is empty".to_string()));
    }

    let user = match UserUpdate::new(id, name, lastname) {
        Ok(u) => u,
        Err(e) => return Err((400, e)),
    };

    repository::update_user(user).await
}
