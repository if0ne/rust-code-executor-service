#![warn(clippy::all)]
#![warn(rust_2018_idioms)]
mod executors;
mod measure;
mod routes;

use crate::routes::compile::compile;
use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};
use dotenv::dotenv;
use paperclip::actix::OpenApiExt;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    dotenv().ok();
    let port = env::var("RUST_SERVICE_PORT")
        .unwrap()
        .parse::<u16>()
        .unwrap();
    let host_address = env::var("RUST_SERVICE_HOST").unwrap();

    HttpServer::new(|| {
        let cors = Cors::default()
            .allow_any_origin()
            .allowed_methods(vec!["POST"])
            .supports_credentials();

        App::new()
            .wrap_api()
            .wrap(Logger::default())
            .wrap(cors)
            .service(compile)
            .with_json_spec_at("/api/spec/v2")
            .with_swagger_ui_at("/swagger-ui")
            .build()
    })
    .bind((host_address, port))?
    .run()
    .await
}
