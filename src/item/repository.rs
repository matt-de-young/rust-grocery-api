#![allow(proc_macro_derive_resolution_fallback)]

use diesel;
use diesel::prelude::*;

use crate::item::model::Item;
use crate::item::model::NewItem;
use crate::schema::items;
use crate::schema::items::dsl::*;
use crate::user::model::User;

pub fn create_item(new_item: NewItem, user: User, connection: &PgConnection) -> QueryResult<Item> {
    diesel::insert_into(items::table)
        .values((&new_item, user_id.eq(user.id)))
        .get_result(connection)
}

pub fn list_items(user: User, connection: &PgConnection) -> QueryResult<Vec<Item>> {
    items
        .filter(user_id.eq(user.id))
        .limit(5)
        .load::<Item>(&*connection)
}

pub fn get_item(item_id: i32, user: Option<User>, connection: &PgConnection) -> QueryResult<Item> {
    if let Some(user) = user {
        return items
            .filter(id.eq(item_id))
            .filter(user_id.eq(user.id))
            .first::<Item>(&*connection);
    } else {
        return items::table.find(item_id).get_result::<Item>(connection);
    }
}

pub fn update_item(
    item_id: i32,
    item: Item,
    user: Option<User>,
    connection: &PgConnection,
) -> QueryResult<Item> {
    match get_item(item_id, user, &connection) {
        Ok(v) => diesel::update(&v).set(&item).get_result(connection),
        Err(e) => Err(e),
    }
}

pub fn delete_item(
    item_id: i32,
    user: Option<User>,
    connection: &PgConnection,
) -> QueryResult<usize> {
    match get_item(item_id, user, &connection) {
        Ok(v) => diesel::delete(&v).execute(connection),
        Err(e) => Err(e),
    }
}
