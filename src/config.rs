use std::env;

pub(crate) struct Config {
    pub(crate) database_url: String,
    pub(crate) port: u16,
}

impl Config {
    pub(crate) fn load() -> Result<Self, Box<dyn std::error::Error>> {
        match dotenvy::dotenv() {
            Ok(_) => {}
            Err(dotenvy::Error::Io(error)) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(error) => return Err(error.into()),
        }

        Ok(Self {
            database_url: env::var("DATABASE_URL")?,
            port: env::var("PORT")
                .unwrap_or_else(|_| "3002".to_string())
                .parse()?,
        })
    }
}
