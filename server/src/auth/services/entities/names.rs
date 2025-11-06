#[derive(Debug)]
pub struct Names {
    names: String,
}

impl ToString for Names {
    fn to_string(&self) -> String {
        self.names.clone()
    }
}

impl Names {
    pub fn new(mut names: &str) -> Result<Self, &str> {
        names = names.trim();

        if names.is_empty() {
            return Err("Names is empty");
        }

        let formated_name: String = names
            .split_whitespace()
            .map(|name| {
                if name.chars().any(|c| !c.is_alphabetic()) {
                    return Err("Names must by contains alphabetic chars");
                };

                let mut chars = name.chars();

                let first = chars.next().unwrap().to_uppercase();
                let full_name = first.to_string() + chars.as_str();
                Ok(full_name)
            })
            .collect::<Result<Vec<String>, &str>>()?
            .join(" ");

        Ok(Names {
            names: formated_name,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_names() {
        let names = Names::new("Rober").unwrap();
        assert_eq!(names.names, "Rober");

        let names = Names::new("Rober  ").unwrap();
        assert_eq!(names.names, "Rober");

        let names = Names::new(" ");
        assert!(names.is_err());

        let names = Names::new("Rob2er torres");
        assert!(names.is_err());

        let names = Names::new("Robe2r 2");
        assert!(names.is_err());

        let names = Names::new("");
        assert!(names.is_err());

        let names = Names::new("ártos").unwrap();
        assert_eq!("Ártos", names.names);
    }
}
