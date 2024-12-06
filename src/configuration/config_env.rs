use crate::handle_error::error::Error;

#[derive(Debug, Clone)]
pub struct DatabaseConfig {
  pub url: String,
  pub database_namespace: String,
  pub database_name: String,
}

#[derive(Debug, Clone)]
pub struct ServerConfig {
  pub host: String,
  pub port: u16,
}

#[derive(Debug, Clone)]
pub struct Config {
  pub database: DatabaseConfig,
  pub server: ServerConfig,
}

impl Config {
  pub fn from_env() -> Result<Self, Error> {
    Ok(Self {
      database: DatabaseConfig {
        url: std::env::var("DATABASE_URL").map_err(|_| Error::ConfigurationError("DATABASE_URL not set".to_string()))?,
        database_namespace: std::env::var("DATABASE_NAMESPACE").map_err(|_| Error::ConfigurationError("SURREAL_DB_NAMESPACE must be set".to_string()))?,
        database_name: std::env::var("DATABASE_NAME").map_err(|_| Error::ConfigurationError("SURREAL_DB_DATABASE must be set".to_string()))?,
      },
      server: ServerConfig {
        host: std::env::var("SERVER_HOST").map_err(|_| Error::ConfigurationError("SERVER_HOST must be set".to_string()))?,
        port: std::env::var("SERVER_PORT")
          .map_err(|_| Error::ConfigurationError("SERVER_PORT must be set".to_string()))?
          .parse().map_err(|_| Error::ConfigurationError("SERVER_PORT must be a valid number".to_string()))?,
      },
    })
  }
}