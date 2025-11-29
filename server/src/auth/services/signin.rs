use jsonwebtoken::{EncodingKey, Header, encode};
use serde::Serialize;

use crate::{
    auth::services::{entities::user::UserSession, repository::SigninRepository},
    config::token::TokenVars,
    shared::etities::userdb::{User, UserClaims},
};

// Service
pub struct SigninService;

impl SigninService {
    pub async fn signin(username: &str, password: &str) -> Result<impl Serialize, (u16, String)> {
        let secret = match TokenVars::from_env() {
            Ok(s) => s,
            Err(e) => return Err((500, e)),
        };

        let user: User = SigninRepository::new().signin(username, password).await?;

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

        Ok({
            UserSession {
                id: user.id,
                username: user.username,
                token: token,
                name: user.name,
                lastname: user.lastname,
            }
        })
    }
}
