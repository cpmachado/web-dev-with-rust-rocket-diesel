use crate::schema::rustaceans;
use diesel::{AsChangeset, Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Queryable, Selectable, AsChangeset)]
#[diesel(table_name = rustaceans)]
pub struct Rustacean {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub created_at: String,
}

#[derive(Deserialize, Insertable, AsChangeset)]
#[diesel(table_name = rustaceans)]
pub struct NewRustacean {
    pub name: String,
    pub email: String,
}
