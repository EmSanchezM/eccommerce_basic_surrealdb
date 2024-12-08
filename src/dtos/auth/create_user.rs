use serde::{Deserialize, Serialize};

use crate::models::user::Address;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUser {
  pub email: String,
  pub password: String,
  pub phone_number: Option<String>,
  pub company_name: Option<String>,
  pub address: Option<Address>,
}