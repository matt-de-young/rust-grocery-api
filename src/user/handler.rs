use std::env;

use diesel::result::Error;
use rocket::http::Status;
use rocket::response::status;
use rocket_contrib::json::Json;

use crate::connection::DbConn;
use crate::user;
use crate::user::model::User;
use crate::user::model::NewUser;
use crate::user::model::LoginCredentials;
use crate::user::model::ResponseToken;


#[post("/signup", format = "application/json", data = "<new_user>")]
pub fn signup(new_user: Json<NewUser>, connection: DbConn) -> Result<status::Created<Json<User>>, Status> {
    user::repository::create_user(new_user.into_inner(), &connection)
        .map(|user| user_created(user))
        .map_err(|error| error_status(error))
}

#[post("/login", format = "application/json", data = "<login>")]
pub fn login(login: Json<LoginCredentials>, connection: DbConn) -> Result<Json<ResponseToken>, Status> {
    if let Some(result) = user::repository::login(login.into_inner(), &connection) {
        let token = user::model::generate_token(result);
        Result::Ok(Json(token))
    } else {
        Result::Err(Status::Unauthorized)
    }
}

fn user_created(user: User) -> status::Created<Json<User>> {
    status::Created(
        format!("{host}:{port}/user/{id}", host = host(), port = port(), id = user.id).to_string(),
        Some(Json(user)))
}

fn host() -> String {
    env::var("ROCKET_ADDRESS").expect("ROCKET_ADDRESS must be set")
}

fn port() -> String {
    env::var("ROCKET_PORT").expect("ROCKET_PORT must be set")
}

fn error_status(error: Error) -> Status {
    match error {
        Error::NotFound => Status::NotFound,
        _ => Status::InternalServerError
    }
}
