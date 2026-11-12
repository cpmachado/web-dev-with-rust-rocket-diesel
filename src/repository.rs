use diesel::prelude::*;

use crate::{
    models::{NewRustacean, Rustacean},
    schema::rustaceans,
};

pub struct RustaceanRepository;

impl RustaceanRepository {
    pub fn find(c: &mut SqliteConnection, id: i32) -> QueryResult<Rustacean> {
        rustaceans::table.find(id).get_result::<Rustacean>(c)
    }

    pub fn find_multiple(c: &mut SqliteConnection, limit: i64) -> QueryResult<Vec<Rustacean>> {
        rustaceans::table
            .order(rustaceans::id.desc())
            .limit(limit)
            .load::<Rustacean>(c)
    }

    pub fn create(c: &mut SqliteConnection, new_rustacean: NewRustacean) -> QueryResult<Rustacean> {
        diesel::insert_into(rustaceans::table)
            .values(new_rustacean)
            .returning(Rustacean::as_returning())
            .get_result(c)
    }

    pub fn save(c: &mut SqliteConnection, id: i32, rustacean: Rustacean) -> QueryResult<Rustacean> {
        diesel::update(rustaceans::table.find(id))
            .set(rustacean)
            .returning(Rustacean::as_returning())
            .get_result(c)
    }

    pub fn delete(c: &mut SqliteConnection, id: i32) -> QueryResult<Rustacean> {
        diesel::delete(rustaceans::table.find(id))
            .returning(Rustacean::as_returning())
            .get_result(c)
    }
}
