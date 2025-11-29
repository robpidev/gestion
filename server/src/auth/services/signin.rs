use jsonwebtoken::{EncodingKey, Header, encode};
use serde::{Deserialize, Serialize};

use crate::{
    auth::services::{entities::user::UserSession, repository::SigninRepository},
    shared::etities::userdb::User,
};

// Jwt

#[derive(Debug, Serialize, Deserialize)]
struct Claims<'a> {
    username: &'a str,
    name: &'a str,
    lastname: &'a str,
    id: &'a str,
    exp: usize,
}

// Service
pub struct SigninService;

impl SigninService {
    pub async fn signin(username: &str, password: &str) -> Result<impl Serialize, (u16, String)> {
        let user: User = SigninRepository::new().signin(username, password).await?;

        let claims = Claims {
            id: &user.id,
            lastname: &user.lastname,
            username: &user.username,
            name: &user.name,
            exp: 0,
        };

        let token = match encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret("clave".as_ref()),
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
