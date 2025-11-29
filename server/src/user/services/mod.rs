mod entities;
mod repository;
mod username;
use entities::UserUpdate;
use serde::Serialize;

use repository::NameLastNameUpdateRepository;
pub use username::UsernameService;

pub struct UserUpdateService;

impl UserUpdateService {
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

        let mut repo = NameLastNameUpdateRepository::new();

        let new_user = repo.update_user(user).await;

        new_user
    }
}
