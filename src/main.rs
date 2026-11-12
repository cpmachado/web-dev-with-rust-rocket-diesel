#[macro_use]
extern crate rocket;

mod auth;
mod models;
mod schema;

use auth::BasicAuth;
use diesel::prelude::*;
use models::{NewRustacean, Rustacean};
use rocket::{http::Status, response::status::Custom};
use rocket::{response::status, serde::json::{Json, Value, json}};
use rocket_sync_db_pools::database;
use schema::rustaceans;

#[database("sqlite")]
struct DbConn(SqliteConnection);

#[get("/rustaceans")]
async fn get_rustaceans(_auth: BasicAuth, db: DbConn) -> Result<Value, Custom<Value>> {
    db.run(|c| {
        rustaceans::table
            .order(rustaceans::id.desc())
            .limit(1000)
            .load::<Rustacean>(c)
    })
    .await
    .map(|rustaceans| json!(rustaceans))
    .map_err(|e| {
        status::Custom(
            Status::InternalServerError,
            json!({ "error": e.to_string() }),
        )
    })
}

#[get("/rustaceans/<id>")]
async fn get_rustacean_by_id(
    id: i32,
    _auth: BasicAuth,
    db: DbConn,
) -> Result<Value, Custom<Value>> {
    db.run(move |c| rustaceans::table.find(id).get_result::<Rustacean>(c))
        .await
        .map(|r| json!(r))
        .map_err(|e| match e {
            diesel::result::Error::NotFound => {
                status::Custom(Status::NotFound, json!({ "error": "not found" }))
            }
            e => status::Custom(
                Status::InternalServerError,
                json!({ "error": e.to_string()}),
            ),
        })
}

#[post("/rustaceans", format = "json", data = "<new_rustacean>")]
async fn create_rustacean(
    _auth: BasicAuth,
    db: DbConn,
    new_rustacean: Json<NewRustacean>,
) -> Value {
    db.run(|c| {
        let result = diesel::insert_into(rustaceans::table)
            .values(new_rustacean.into_inner())
            .execute(c)
            .expect("DB error when inserting");
        json!(result)
    })
    .await
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

#[catch(422)]
fn unprocessable_entity() -> Value {
    json!("Unprocessable Entity")
}

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount(
            "/",
            routes![
                get_rustaceans,
                get_rustacean_by_id,
                create_rustacean,
                update_rustacean,
                delete_rustacean
            ],
        )
        .register(
            "/",
            catchers![not_found, unauthorized, unprocessable_entity],
        )
        .attach(DbConn::fairing())
        .launch()
        .await;
}
