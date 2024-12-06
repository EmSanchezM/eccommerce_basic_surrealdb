use dotenv::dotenv;
use std::env;

mod database;
mod handle_error;

use actix_web::{web, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  dotenv().ok();

  let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
  let database_name = env::var("DATABASE_NAME").expect("DATABASE_NAME must be set");
  let database_namespace = env::var("DATABASE_NAMESPACE").expect("DATABASE_NAMESPACE must be set");

  let db = database::connection::DatabaseConnection::new(&database_url, &database_name, &database_namespace)
      .await
      .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    
  let database = db.get_client().clone();

  HttpServer::new(move || {
    App::new()
      .app_data(web::Data::new(database.clone()))
      
  })
  .bind("127.0.0.1:8080")?
  .run()
  .await
}
