use actix_web::{web, App, HttpServer};
use actix_web::middleware::Logger;

use crate::configuration::config_env::Config;
use crate::database::connection::DatabaseConnection;

pub async fn run(config: Config) -> std::io::Result<()> {
  let config_init = config.clone();
  let address = (config_init.server.host.as_str(), config_init.server.port);

  let db = DatabaseConnection::new(
    &config_init.database.url.as_str(),
    &config_init.database.database_name.as_str(),
    &config_init.database.database_namespace.as_str()
  )
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