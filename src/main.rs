#![warn(clippy::all)]
#![warn(rust_2018_idioms)]
mod executors;
mod mesure;
mod routes;

#[macro_use]
extern crate rocket;

use crate::mesure::ProcessInformer;
use crate::routes::compile::compile;

use dotenv::dotenv;
use std::env;

use rocket::http::{Method, Status};
use rocket::response::status;
use rocket::Config;
use rocket_cors::{AllowedOrigins, CorsOptions};

#[get("/")]
async fn index() -> status::Custom<String> {
    status::Custom(Status::Ok, String::from("All is ok or not"))
}

#[rocket::main]
async fn main() {
    dotenv().ok();
    let port = env::var("RUST_SERVICE_PORT")
        .unwrap()
        .parse::<u16>()
        .unwrap();
    let host_address = env::var("RUST_SERVICE_HOST").unwrap();

    let cors = CorsOptions::default()
        .allowed_origins(AllowedOrigins::all())
        .allowed_methods(
            vec![Method::Get, Method::Post]
                .into_iter()
                .map(From::from)
                .collect(),
        )
        .allow_credentials(true);

    let config = Config::figment()
        .merge(("address", host_address))
        .merge(("port", port));

    rocket::custom(config)
        .attach(cors.to_cors().unwrap())
        .mount("/", routes![index, compile])
        .launch()
        .await
        .unwrap();
}
