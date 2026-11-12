#[macro_use]
extern crate rocket;

mod auth;
mod models;
mod repository;
mod schema;

use auth::BasicAuth;
use diesel::prelude::*;
use models::{NewRustacean, Rustacean};
use repository::RustaceanRepository;
use rocket::{http::Status, response::status::Custom};
use rocket::{
    response::status,
    serde::json::{Json, Value, json},
};
use rocket_sync_db_pools::database;

#[database("sqlite")]
struct DbConn(SqliteConnection);

#[get("/rustaceans")]
async fn get_rustaceans(_auth: BasicAuth, db: DbConn) -> Result<Value, Custom<Value>> {
    db.run(|c| RustaceanRepository::find_multiple(c, 100))
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
    db.run(move |c| RustaceanRepository::find(c, id))
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
) -> Result<status::Custom<Value>, status::Custom<Value>> {
    db.run(move |c| RustaceanRepository::create(c, new_rustacean.into_inner()))
        .await
        .map(|id| status::Custom(Status::Created, json!(id)))
        .map_err(|e| {
            status::Custom(
                Status::InternalServerError,
                json!({ "error": e.to_string()}),
            )
        })
}

#[put("/rustaceans/<id>", format = "json", data = "<rustacean>")]
async fn update_rustacean(
    id: i32,
    _auth: BasicAuth,
    db: DbConn,
    rustacean: Json<Rustacean>,
) -> Result<Value, status::Custom<Value>> {
    db.run(move |c| RustaceanRepository::save(c, id, rustacean.into_inner()))
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

#[delete("/rustaceans/<id>")]
async fn delete_rustacean(
    id: i32,
    _auth: BasicAuth,
    db: DbConn,
) -> Result<Value, status::Custom<Value>> {
    db.run(move |c| RustaceanRepository::delete(c, id))
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
