use actix_web::{web, HttpResponse, get, post};
use crate::controllers::user_controller::UserController;
use crate::database::connection::DatabaseConnection;
use crate::dtos::auth::create_user::CreateUser;
use crate::handle_error::error::ErrorResponse;
use crate::services::user_service::UserService;

#[get("/")]
async fn find_all(database: web::Data<DatabaseConnection>) -> HttpResponse {
  match UserController::new(UserService::new(&database)).find_all().await {
    Ok(users) => HttpResponse::Ok().json(users),
    Err(err) => HttpResponse::InternalServerError().body(err.to_string())
  }
}

#[post("/register")]
async fn register(database: web::Data<DatabaseConnection>, user: web::Json<CreateUser>) -> HttpResponse {
  let user_dto = user.into_inner();

  match UserController::new(UserService::new(&database)).create_user(user_dto).await {
    Ok(user) => HttpResponse::Ok().json(user),
    Err(err) => HttpResponse::InternalServerError().json(ErrorResponse {
      code: 500,
      message: err.to_string()
    })
  }
}

pub fn routes(config: &mut web::ServiceConfig) {
  config.service(
    web::scope("/api/v1/users")
      .service(register)
      .service(find_all)
  );
}

