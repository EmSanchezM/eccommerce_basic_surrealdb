use actix_web::{web, App, HttpServer};
use actix_web::middleware::Logger;
use log::{debug, error};

use crate::configuration::config_env::Config;
use crate::database::connection::DatabaseConnection;

use crate::routes::user_routes;

pub async fn run(config: Config) -> std::io::Result<()> {
  let config_init = config.clone();
  let address: (&str, u16) = (&config_init.server.host.as_str(), config_init.server.port);

  debug!("Starting server on {}:{}", config_init.server.host, config_init.server.port);

  let db = DatabaseConnection::new(
    &config_init.database.url.as_str(),
    &config_init.database.database_name.as_str(),
    &config_init.database.database_namespace.as_str(),
    &config_init.database.username.as_str(),
    &config_init.database.password.as_str()
  )
      .await
      .map_err(|e| {
        error!("Error connecting to database: {}", e);
        std::io::Error::new(std::io::ErrorKind::Other, e)
      })?;
    
  let database = web::Data::new(db);
  let configuration = web::Data::new(config.clone());

  let database = database.clone();
  let configuration = configuration.clone();
  
  HttpServer::new(move || {
    App::new()
      .app_data(configuration.clone())
      .app_data(database.clone())
      .wrap(Logger::default())
      .configure(user_routes::routes)
  })
  .bind(address)?
  .run()
  .await
}