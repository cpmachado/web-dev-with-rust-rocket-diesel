#[macro_use]
extern crate rocket;

mod auth;
mod models;
mod schema;

use auth::BasicAuth;
use diesel::prelude::*;
use models::Rustacean;
use rocket::{
    response::status,
    serde::json::{Value, json},
};
use rocket_sync_db_pools::database;
use schema::rustaceans;

#[database("sqlite")]
struct DbConn(SqliteConnection);

#[get("/rustaceans")]
async fn get_rustaceans(_auth: BasicAuth, db: DbConn) -> Value {
    db.run(|c| {
        let rustaceans = rustaceans::table
            .order(rustaceans::id.desc())
            .limit(1000)
            .load::<Rustacean>(c)
            .expect("db error");
        json!(rustaceans)
    })
    .await
}

#[get("/rustaceans/<id>")]
fn get_rustacean(id: i32, _auth: BasicAuth) -> Value {
    json!({"id": id, "name": "John Doe", "email": "john@doe.com" })
}

#[post("/rustaceans", format = "json")]
fn create_rustacean(_auth: BasicAuth) -> Value {
    json!({"id": 3, "name": "John Doe", "email": "john@doe.com" })
}

#[put("/rustaceans/<id>", format = "json")]
fn update_rustacean(id: i32, _auth: BasicAuth) -> Value {
    json!({"id": id, "name": "John Doe", "email": "john@doe.com" })
}

#[delete("/rustaceans/<_id>")]
fn delete_rustacean(_id: i32, _auth: BasicAuth) -> status::NoContent {
    status::NoContent
}

#[catch(404)]
fn not_found() -> Value {
    json!("Not found")
}

#[catch(401)]
fn unauthorized() -> Value {
    json!("Unauthorized")
}

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount(
            "/",
            routes![
                get_rustaceans,
                get_rustacean,
                create_rustacean,
                update_rustacean,
                delete_rustacean
            ],
        )
        .register("/", catchers![not_found, unauthorized])
        .attach(DbConn::fairing())
        .launch()
        .await;
}
