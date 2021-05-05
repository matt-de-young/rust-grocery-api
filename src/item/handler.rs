use std::env;

use diesel::result::Error;
use rocket::http::Status;
use rocket::response::status;
use rocket_contrib::json::Json;

use crate::connection::DbConn;
use crate::item;
use crate::item::model::Item;
use crate::item::model::NewItem;
use crate::user::model::User;

#[get("/")]
pub fn all_items(user: User, connection: DbConn) -> Result<Json<Vec<Item>>, Status> {
    println!("Getting items for user {}", &user.username);
    item::repository::list_items(user, &connection)
        .map(|item| Json(item))
        .map_err(|error| error_status(error))
}

#[post("/", format = "application/json", data = "<new_item>")]
pub fn create_item(
    user: User,
    new_item: Json<NewItem>,
    connection: DbConn,
) -> Result<status::Created<Json<Item>>, Status> {
    println!("here 0 {}", &new_item.body);
    item::repository::create_item(new_item.into_inner(), user, &connection)
        .map(|item| item_created(item))
        .map_err(|error| error_status(error))
}

#[get("/<id>")]
pub fn get_item(id: i32, user: User, connection: DbConn) -> Result<Json<Item>, Status> {
    item::repository::get_item(id, Some(user), &connection)
        .map(|item| Json(item))
        .map_err(|error| error_status(error))
}

#[put("/<id>", format = "application/json", data = "<item>")]
pub fn update_item(
    id: i32,
    item: Json<Item>,
    user: User,
    connection: DbConn,
) -> Result<Json<Item>, Status> {
    item::repository::update_item(id, item.into_inner(), Some(user), &connection)
        .map(|item| Json(item))
        .map_err(|error| error_status(error))
}

#[delete("/<id>")]
pub fn delete_item(id: i32, user: User, connection: DbConn) -> Result<status::NoContent, Status> {
    item::repository::delete_item(id, Some(user), &connection)
        .map(|_| status::NoContent)
        .map_err(|error| error_status(error))
}

fn item_created(item: Item) -> status::Created<Json<Item>> {
    println!("here final");
    status::Created(
        format!(
            "{host}:{port}/item/{id}",
            host = host(),
            port = port(),
            id = item.id
        )
        .to_string(),
        Some(Json(item)),
    )
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
        _ => Status::InternalServerError,
    }
}
