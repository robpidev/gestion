pub struct Username {
    username: String,
}

impl ToString for Username {
    fn to_string(&self) -> String {
        self.username.clone()
    }
}

impl Username {
    pub fn new(username: &str) -> Result<Self, &str> {
        if username.is_empty() {
            return Err("Username is empty");
        };

        if username.len() < 5 || username.len() > 20 {
            return Err("Username must be between 5 and 20 chars");
        }

        if username
            .chars()
            .any(|c| !(c.is_ascii_alphanumeric() || c == '_' || c == '-' || c == '.'))
        {
            return Err("Username not valid");
        };

        Ok(Self {
            username: username.to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_username() {
        let username = Username::new("Robpi").unwrap();
        assert_eq!(username.username, "Robpi");

        let username = Username::new("12Robp...i_2").unwrap();
        assert_eq!(username.username, "12Robp...i_2");

        let username = Username::new("Robpi ");
        assert!(username.is_err());

        let username = Username::new("ro bpi-");
        assert!(username.is_err());
    }
}
