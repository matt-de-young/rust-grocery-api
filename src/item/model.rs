#![allow(proc_macro_derive_resolution_fallback)]

use crate::schema::items;

#[derive(Identifiable, Queryable, AsChangeset, Serialize, Deserialize, Debug)]
#[table_name = "items"]
pub struct Item {
    pub id: i32,
    pub body: String,
    pub description: Option<String>,
    pub quantity: i32,
    pub complete: bool,
    pub user_id: i32,
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "items"]
pub struct NewItem {
    pub body: String,
    pub description: Option<String>,
    pub quantity: i32,
}
