#[macro_use]
extern crate rocket;

use rocket::serde::json::{json, Value};

#[get("/")]
fn hello() -> Value {
    json!("Hello, world!")
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![hello])
}
