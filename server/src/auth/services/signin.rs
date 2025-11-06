use super::repository;
use crate::shared::etities::userdb::User;

pub async fn signin(username: &str, password: &str) -> Result<String, (u16, String)> {
    let user: User = repository::signin::signin(username, password).await?;

    Ok("hola".to_string())
}
