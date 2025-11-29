use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub id: String,
    pub name: String,
    pub lastname: String,
    pub username: String,
}

impl ToString for User {
    fn to_string(&self) -> String {
        format!(
            r#"{{"name":"{}","lastname":"{}","username":"{}"}}"#,
            self.name, self.lastname, self.username
        )
    }
}

pub trait ToUser {
    fn to_user(&self) -> User;
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserClaims<T> {
    pub user: T,
    pub exp: usize,
}
