use std::sync::Arc;
use surrealdb::engine::any;
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;

use crate::handle_error::error::Error;

pub struct DatabaseConnection {
  db: Arc<Surreal<any::Any>>,
}

impl DatabaseConnection {
  pub async fn new(url: &str, username: &str, password: &str, database_name: &str) -> Result<Self, Error> {
    
    let db = any::connect(url).await?;

    db.use_ns("dev").use_db(database_name).await?;

    db.signin(Root {
      username,
      password,
    }).await?;

    Ok(Self { db: Arc::new(db) })
  }

  pub fn get_client(&self) -> Arc<Surreal<any::Any>> {
    self.db.clone()
  }
}