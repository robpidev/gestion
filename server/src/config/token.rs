use super::Vars;
use crate::set_vars;

pub struct TokenVars {
    seed: String,
}

impl Vars for TokenVars {}

impl TokenVars {
    pub fn from_env() -> Result<Self, String> {
        let mut tk = TokenVars {
            seed: String::new(),
        };
        set_vars!(tk.seed, "TOKEN_SEED");
        Ok(tk)
    }

    pub fn seed(&self) -> &str {
        &self.seed
    }
}
