use env_logger::Env;
use dotenv::dotenv;

mod database;
mod handle_error;
mod configuration;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  dotenv().ok();

  env_logger::Builder
    ::from_env(Env::default().default_filter_or("info"))
    .init();

  use crate::configuration::config_env::Config;
  use crate::configuration::server_web;

  let config = Config::from_env().unwrap();

  server_web::run(config).await?;

  Ok(())
}
