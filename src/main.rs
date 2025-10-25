#[macro_use]
extern crate rocket;

mod auth;

use rocket::{
    response::status,
    serde::json::{Value, json},
};

#[get("/rustaceans")]
fn get_rustaceans(_auth: auth::BasicAuth) -> Value {
    json!([{ "id": 1, "name": "John Doe"}, { "id": 2, "name": "John Doe again"}])
}

#[get("/rustaceans/<id>")]
fn get_rustacean(id: i32) -> Value {
    json!({ "id": id, "name": "John Doe", "email": "john@doe.com"})
}

#[post("/rustaceans", format = "json")]
fn create_rustacean() -> Value {
    json!({ "id": 3, "name": "John Doe", "email": "john@doe.com"})
}

#[put("/rustaceans/<id>", format = "json")]
fn update_rustacean(id: i32) -> Value {
    json!({ "id": id, "name": "John Doe", "email": "john@doe.com"})
}

#[delete("/rustaceans/<_id>")]
fn delete_rustacean(_id: i32) -> status::NoContent {
    status::NoContent
}

#[catch(404)]
fn not_found() -> Value {
    json!({"error": "Not Found", "code": 404 })
}

#[launch]
fn rocket() -> _ {
    rocket::build()
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
        .register("/", catchers![not_found])
}
