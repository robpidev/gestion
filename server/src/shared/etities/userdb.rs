use serde::{Deserialize, Serialize, forward_to_deserialize_any};

#[derive(Serialize, Deserialize, Debug)]
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
