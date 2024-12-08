use async_trait::async_trait;
use std::sync::Arc;
use surrealdb::engine::remote::ws::Client;
use surrealdb::Surreal;
use log;

use crate::database::connection::DatabaseConnection;
use crate::dtos::auth::create_user::CreateUser;
use crate::handle_error::error::Error;
use crate::models::user::User;

pub struct UserService {
  database: Arc<Surreal<Client>>,
}

impl UserService {
  pub fn new(connection: &DatabaseConnection) -> Self {
    Self { 
      database:  connection.get_client()
    }
  }
}

#[async_trait]
pub trait UserRepository {
  async fn find_all(&self) -> Result<Vec<User>, Error>;
  async fn create_user(&self, user: CreateUser) -> Result<User, Error>;
}

#[async_trait]
impl UserRepository for UserService {
  async fn find_all(&self) -> Result<Vec<User>, Error> {
    let users: Vec<User> = self.database.select("user").await?;
    
    Ok(users)
  }

  async fn create_user(&self, user: CreateUser) -> Result<User, Error> {
    let user = User::from(user);
    
    let result = self.database
      .query(r#"
        LET $hash_password = CRYPTO::ARGON2::GENERATE($password);
        LET $created_user = (CREATE user CONTENT {
          id: rand::ulid(),
          email: $email,
          password: $hash_password,
          phone_number: $phone_number,
          company_name: $company_name,
          address: $address,
          is_active: $is_active,
          created_at: $created_at,
          updated_at: $updated_at
        });
        RETURN $created_user
      "#)
      .bind(("password", user.password))
      .bind(("email", user.email))
      .bind(("phone_number", user.phone_number))
      .bind(("company_name", user.company_name))
      .bind(("address", user.address))
      .bind(("is_active", user.is_active))
      .bind(("created_at", user.created_at))
      .bind(("updated_at", user.updated_at))
      .await;

    match result {
      Ok(mut response) => {
        // Agregar más logs para depuración
        log::debug!("Response raw: {:?}", response);
        
        match response.take::<Vec<User>>(0) {
          Ok(users) => {
            log::debug!("Users vector: {:?}", users);
            if let Some(created_user) = users.into_iter().next() {
              Ok(created_user)
            } else {
              Err(Error::SurrealDBError(String::from("No user was returned after creation")))
            }
          }
          Err(e) => {
            log::error!("Error taking user from response: {:?}", e);
            Err(Error::SurrealDBError(format!("Failed to parse user from response: {:?}", e)))
          }
        }
      }
      Err(err) => {
        log::error!("Database error: {:?}", err);
        Err(Error::SurrealDBError(format!("Database error: {:?}", err)))
      }
    }
    
  }
}