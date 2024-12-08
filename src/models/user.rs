use surrealdb::RecordId;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

use crate::dtos::auth::create_user::CreateUser;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Address {
  pub city: String,
  pub country: String,
  pub street: String,
  pub address: String,
  pub zip_code: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
  pub id: Option<RecordId>,
  pub email: String,
  pub password: String,
  pub phone_number: Option<String>,
  pub company_name: Option<String>,
  pub address: Option<Address>,
  pub is_active: bool,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
}

impl From<CreateUser> for User {
  fn from(user: CreateUser) -> Self {
    Self {
      id: None,
      email: user.email,
      password: user.password,
      phone_number: user.phone_number,
      company_name: user.company_name,
      address: user.address,
      is_active: true,
      created_at: Utc::now(),
      updated_at: Utc::now(),
    }
  }
}