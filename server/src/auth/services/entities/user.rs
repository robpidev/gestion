use serde::Serialize;

// use super::names::Names;
// use super::username::Username;
use crate::shared::etities::{names::Names, username::Username};

pub struct NewUser {
    name: Names,
    lastname: Names,
    username: Username,
    password: String,
}

impl NewUser {
    pub fn new<'a>(
        names: &'a str,
        lastname: &'a str,
        username: &'a str,
        password: &'a str,
    ) -> Result<Self, &'a str> {
        Ok(Self {
            name: Names::new(names)?,
            lastname: Names::new(lastname)?,
            username: Username::new(username)?,
            password: password.to_string(),
        })
    }

    pub fn name(&self) -> String {
        self.name.to_string()
    }

    pub fn lastname(&self) -> String {
        self.lastname.to_string()
    }

    pub fn username(&self) -> String {
        self.username.to_string()
    }

    pub fn password(&self) -> String {
        self.password.to_string()
    }
}

impl ToString for NewUser {
    fn to_string(&self) -> String {
        format!(
            r#"{{"name":"{}","lastname":"{}","username":"{}","password":"{}"}}"#,
            self.name.to_string(),
            self.lastname.to_string(),
            self.username.to_string(),
            self.password
        )
    }
}

#[derive(Serialize)]
pub struct UserSession {
    pub id: String,
    pub username: String,
    pub name: String,
    pub lastname: String,
    pub token: String,
}
