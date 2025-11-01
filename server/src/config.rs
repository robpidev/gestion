pub struct Config {
    pub host: String,
    pub port: u16,
}

impl Config {
    pub fn from_env() -> Result<Config, String> {
        if let Err(e) = dotenvy::dotenv() {
            return Err(format!("Failed to load .env: {}", e));
        };

        let host = Config::var("HOST")?;
        let port = Config::var("PORT")?
            .parse()
            .map_err(|e| format!("PORT: {}", e))?;

        Ok(Config { host, port })
    }

    fn var(name: &str) -> Result<String, String> {
        match std::env::var(name) {
            Ok(val) => Ok(val),
            Err(e) => Err(format!("{}: {}", name, e)),
        }
    }
}
