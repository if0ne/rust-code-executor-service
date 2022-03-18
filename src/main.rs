#![warn(clippy::all)]
#![warn(rust_2018_idioms)]
mod executors;
mod mesure;
mod routes;

#[macro_use]
extern crate rocket;

use crate::mesure::ProcessInformer;
use crate::routes::compile::compile;
use crate::routes::compile::okapi_add_operation_for_compile_;

use dotenv::dotenv;
use std::env;

use rocket::http::Method;
use rocket::Config;
use rocket_cors::{AllowedOrigins, CorsOptions};
use rocket_okapi::{openapi_get_routes, swagger_ui::*};

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
        .mount("/", openapi_get_routes![compile])
        .mount(
            "/swagger-ui/",
            make_swagger_ui(&SwaggerUIConfig {
                url: "../openapi.json".to_owned(),
                ..Default::default()
            }),
        )
        .launch()
        .await
        .unwrap();
}
