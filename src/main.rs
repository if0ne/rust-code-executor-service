#[macro_use] extern crate rocket;

use rocket::http::Status;
use rocket::response::{status};

#[get("/")]
async fn index<'o>() -> status::Custom<String> {
    status::Custom(Status::Ok, String::from("test"))
}

#[rocket::main]
async fn main() {
    rocket::build()
        .mount("/", routes![index])
        .launch()
        .await;
}