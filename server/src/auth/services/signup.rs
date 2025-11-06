use super::entities::user::NewUser;
use super::repository;

pub async fn register(
    name: &str,
    lastname: &str,
    username: &str,
    password: &str,
) -> Result<String, (u16, String)> {
    let user = match NewUser::new(&name, &lastname, &username, &password) {
        Ok(user) => user,
        Err(err) => return Err((400, err.to_string())),
    };

    match repository::register::create_user(user).await {
        Ok(user_id) => Ok(user_id),
        Err(err) => Err(err),
    }
}
