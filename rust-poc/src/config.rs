use std::env;

#[derive(Clone)]
pub struct Config {
    pub db_uri: String,
    pub cache_control: String,
    pub port: u16,
}

impl Config {
    pub fn from_env() -> Result<Self, String> {
        let db_uri = env::var("DBURI")
            .map_err(|_| "DBURI environment variable not set".to_string())?;

        let cache_control = env::var("CACHE_CONTROL")
            .unwrap_or_else(|_| "public, max-age=90".to_string());

        let port = env::var("PORT")
            .ok()
            .and_then(|p| p.parse().ok())
            .unwrap_or(9010);

        Ok(Config {
            db_uri,
            cache_control,
            port,
        })
    }
}
