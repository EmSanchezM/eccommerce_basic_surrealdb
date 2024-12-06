use actix_web::{web, App, HttpServer};
use actix_web::middleware::Logger;

use crate::configuration::config_env::Config;
use crate::database::connection::DatabaseConnection;

pub async fn run(config: Config) -> std::io::Result<()> {
  let config_init = config.clone();

  let database_url = &config_init.database.url;
  let database_name = &config_init.database.database_name;
  let database_namespace = &config_init.database.database_namespace;
  let address = (config_init.server.host.as_str(), config_init.server.port);

  let db = DatabaseConnection::new(database_url, database_name, database_namespace)
      .await
      .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    
  let database = db.get_client().clone();

  HttpServer::new(move || {
    App::new()
      .app_data(web::Data::new(config.clone()))
      .app_data(web::Data::new(database.clone()))
      .wrap(Logger::default())
  })
  .bind(address)?
  .run()
  .await
}