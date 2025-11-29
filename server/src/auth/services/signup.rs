use serde::Serialize;

use super::entities::user::NewUser;

use super::repository::RegisterRepository;

pub async fn register(
    name: &str,
    lastname: &str,
    username: &str,
    password: &str,
) -> Result<impl Serialize, (u16, String)> {
    let user = match NewUser::new(&name, &lastname, &username, &password) {
        Ok(user) => user,
        Err(err) => return Err((400, err.to_string())),
    };

    RegisterRepository::new().register_user(user).await
}
