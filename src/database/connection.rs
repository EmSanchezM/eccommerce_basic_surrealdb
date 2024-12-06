use std::sync::Arc;
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::Surreal;

use crate::handle_error::error::Error;

pub struct DatabaseConnection {
  db: Arc<Surreal<Client>>,
}

impl DatabaseConnection {
  pub async fn new(url: &str, database_name: &str, database_namespace: &str) -> Result<Self, Error> {
    
    let db: Surreal<Client> = Surreal::init();

    let _  = db.connect::<Ws>(url).await?;

    db.use_ns(database_namespace).use_db(database_name).await?;

    Ok(Self { db: Arc::new(db) })
  }

  pub fn get_client(&self) -> Arc<Surreal<Client>> {
    self.db.clone()
  }
}