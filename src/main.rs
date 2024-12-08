use env_logger;
use env_logger::Env;
use dotenv::dotenv;
use log::error;

mod database;
mod handle_error;
mod configuration;
mod models;
mod dtos;
mod services;
mod controllers;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  dotenv().ok();

  env_logger::Builder
    ::from_env(Env::default().default_filter_or("info"))
    .init();

  use crate::configuration::config_env::Config;
  use crate::configuration::server_web;

  let config = Config::from_env().map_err(|err| {
    error!("Configuration error: {}", err);
    std::io::Error::new(std::io::ErrorKind::Other, err)
  })?;

  server_web::run(config).await?;

  Ok(())
}
