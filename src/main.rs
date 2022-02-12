mod mesure;

#[macro_use]
extern crate rocket;

use crate::mesure::ProcessInformer;
use rocket::http::Status;
use rocket::response::status;

#[get("/")]
async fn index<'o>() -> status::Custom<String> {
    let mut process = std::process::Command::new("rustc")
        .arg("test.txt")
        .spawn()
        .unwrap();
    let info = process.get_process_info();
    status::Custom(Status::Ok, format!("{:?}", info))
}

#[rocket::main]
async fn main() {
    rocket::build()
        .mount("/", routes![index])
        .launch()
        .await
        .unwrap();
}
