#![feature(decl_macro, proc_macro_hygiene)]
#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate r2d2;
extern crate r2d2_diesel;
#[macro_use]
extern crate rocket;
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;

use dotenv::dotenv;

mod connection;
mod item;
mod post;
mod schema;
mod user;

fn main() {
    dotenv().ok();
    rocket::ignite()
        .manage(connection::init_pool())
        .mount("/", routes![user::handler::signup, user::handler::login,])
        .mount(
            "/items",
            routes![
                item::handler::all_items,
                item::handler::create_item,
                item::handler::get_item,
                item::handler::update_item,
                item::handler::delete_item
            ],
        )
        .mount(
            "/posts",
            routes![
                post::handler::all_posts,
                post::handler::create_post,
                post::handler::get_post,
                post::handler::update_post,
                post::handler::delete_post
            ],
        )
        .launch();
}
