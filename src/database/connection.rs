
use std::sync::Arc;
use surrealdb::engine::remote::ws::Ws;
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;
use surrealdb::engine::remote::ws::Client;

use crate::handle_error::error::Error;

pub struct DatabaseConnection {
  db: Arc<Surreal<Client>>,
}

impl DatabaseConnection {
  pub async fn new(url: &str, username: &str, password: &str, database_name: &str) -> Result<Self, Error> {
    let db: Surreal<Client> = Surreal::init();

    let _  = db.connect::<Ws>(url).await?;

    db.use_ns("").use_db(database_name).await?;

    db.signin(Root {
      username,
      password,
    }).await?;

    match db.health().await {
      Ok(_) => {
          println!("SurrealDB is healthy!");
      },
      Err(e) => {
          println!("SurrealDB is not healthy: {}", e);
      }
    }

    Ok(Self { db: Arc::new(db) })
  }

  pub fn get_client(&self) -> Arc<Surreal<Client>> {
    self.db.clone()
  }
}