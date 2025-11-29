use crate::shared::etities::names::Names;

macro_rules! set_format {
    ($type:ty, $opt:expr) => {
        if let Some(val) = $opt {
            match <$type>::new(&val) {
                Ok(v) => Some(v),
                Err(e) => return Err(e.to_string()),
            }
        } else {
            None
        }
    };
}

pub struct UserUpdate {
    pub id: String,
    name: Option<Names>,
    lastname: Option<Names>,
}

impl ToString for UserUpdate {
    fn to_string(&self) -> String {
        macro_rules! concat_json {
            ($var:expr, $attr:ident) => {
                if let Some(val) = &self.$attr {
                    $var += &format!(r#""{}":""#, stringify!($attr));
                    $var.push_str(&val.to_string());
                    $var.push_str(r#"","#);
                }
            };
        }

        let mut user = r#"{"#.to_string();
        concat_json!(user, name);
        concat_json!(user, lastname);
        user.pop();
        user.push_str(r#"}"#);

        user
    }
}

impl UserUpdate {
    pub fn new(id: String, name: Option<String>, lastname: Option<String>) -> Result<Self, String> {
        Ok(UserUpdate {
            id,
            name: set_format!(Names, name),
            lastname: set_format!(Names, lastname),
        })
    }

    pub fn get_name(&self) -> Option<String> {
        if let Some(val) = &self.name {
            Some(val.to_string())
        } else {
            None
        }
    }

    pub fn get_lastname(&self) -> Option<String> {
        if let Some(val) = &self.lastname {
            Some(val.to_string())
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_update() {
        let user = UserUpdate::new("".to_string(), None, None);
        assert!(user.is_ok());
    }
}
