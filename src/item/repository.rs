#![allow(proc_macro_derive_resolution_fallback)]

use diesel;
use diesel::prelude::*;

use crate::item::model::Item;
use crate::item::model::NewItem;

use crate::schema::items;
use crate::schema::items::dsl::*;

pub fn create_item(new_item: NewItem, conn: &PgConnection) -> QueryResult<Item> {
    diesel::insert_into(items::table)
        .values(&new_item)
        .get_result(conn)
}

pub fn show_items(connection: &PgConnection) -> QueryResult<Vec<Item>> {
    //items.filter(published.eq(true))
    items.limit(5).load::<Item>(&*connection)
}

pub fn get_item(item_id: i32, connection: &PgConnection) -> QueryResult<Item> {
    items::table.find(item_id).get_result::<Item>(connection)
}

pub fn update_item(item_id: i32, item: Item, connection: &PgConnection) -> QueryResult<Item> {
    diesel::update(items::table.find(item_id))
        .set(&item)
        .get_result(connection)
}

pub fn delete_item(item_id: i32, connection: &PgConnection) -> QueryResult<usize> {
    diesel::delete(items::table.find(item_id)).execute(connection)
}
