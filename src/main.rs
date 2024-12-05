use dotenv::dotenv;
use std::env;

mod database;
mod handle_error;

use actix_web::{web, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  dotenv().ok();

  let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
  let database_username = env::var("DATABASE_USERNAME").expect("DATABASE_USERNAME must be set");
  let database_password = env::var("DATABASE_PASSWORD").expect("DATABASE_PASSWORD must be set");
  let database_name = env::var("DATABASE_NAME").expect("DATABASE_NAME must be set");

  let db = database::connection::DatabaseConnection::new(&database_url, &database_username, &database_password, &database_name)
      .await
      .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    
  let database = db.get_client().clone();

  println!("Starting server...");
  println!("DATABASE_URL: {}", database_url);
  println!("Server running at http://127.0.0.1:8080/");

  HttpServer::new(move || {
    App::new()
      .app_data(web::Data::new(database.clone()))
  })
  .bind("127.0.0.1:8080")?
  .run()
  .await
}
