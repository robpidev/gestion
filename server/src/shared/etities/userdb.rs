use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    name: String,
    lastname: String,
    username: String,
}

impl ToString for User {
    fn to_string(&self) -> String {
        format!(
            r#"{{"name":"{}","lastname":"{}","username":"{}"}}"#,
            self.name, self.lastname, self.username
        )
    }
}
