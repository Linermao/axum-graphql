use std::env;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub log_level: String,
    pub database_url: String,
    pub listen_url: String,
}

impl AppConfig {
    pub fn from_env() -> Self {
        let log_level = env::var("RUST_LOG")
            .unwrap_or_else(|_| {
                "info".to_string()
            });

        let database_url = env::var("DATABASE_URL")
            .unwrap_or_else(|_| {
                "postgres://user_name:passwd@localhost:5432/postgres".to_string()
            });

        let listen_url = env::var("LISTEN_URL")
            .unwrap_or_else(|_| {
                "0.0.0.0:3000".to_string()
            });

        Self { 
            log_level,
            database_url,
            listen_url,
        }
    }
}
