use crate::dtos::auth::create_user::CreateUser;
use crate::handle_error::error::Error;
use crate::models::user::User;
use crate::services::user_service::UserRepository;

pub struct UserController<U> where U: UserRepository {
  user_service: U,
}

impl<U> UserController<U> where U: UserRepository {
  pub fn new(user_service: U) -> Self {
    Self { user_service }
  }

  pub async fn find_all(&self) -> Result<Vec<User>, Error> {
    self.user_service.find_all().await
  }

  pub async fn create_user(&self, user: CreateUser) -> Result<User, Error> {
    self.user_service.create_user(user).await
  }
}