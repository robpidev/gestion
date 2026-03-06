mod entities;
mod repository;
mod username;
use entities::UserUpdate;
use jsonwebtoken::{EncodingKey, Header, encode};
use serde::Serialize;

use crate::auth::services::entities::user::UserSession;
use crate::config::token::TokenVars;
use crate::shared::etities::userdb::{User, UserClaims};
use repository::NameLastNameUpdateRepository;
use repository::UserRepository;
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

pub struct UserService;

impl UserService {
    pub async fn refresh(id: String) -> Result<impl Serialize, (u16, String)> {
        let secret = match TokenVars::from_env() {
            Ok(s) => s,
            Err(e) => return Err((500, e)),
        };

        let user: User = UserRepository::get_user_by_id(id).await?;

        let claims = UserClaims {
            user: &user,
            exp: 0,
        };

        let token = match encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(secret.seed().as_ref()),
        ) {
            Ok(t) => t,
            Err(e) => return Err((500, e.to_string())),
        };

        Ok(UserSession {
            id: user.id,
            username: user.username,
            token,
            name: user.name,
            lastname: user.lastname,
        })
    }
}
