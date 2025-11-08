use serde::{Deserialize, Serialize};

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
