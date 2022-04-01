#![warn(clippy::all)]
#![warn(rust_2018_idioms)]
#![feature(thread_is_running)]
mod executors;
mod measure;
mod routes;

use crate::routes::secret_key::SecretKey;
use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};
use paperclip::actix::{web, OpenApiExt};
use routes::execute_service::route::execute;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));
    dotenv::dotenv().ok();
    let port = std::env::var("RUST_SERVICE_PORT")
        .unwrap()
        .parse::<u16>()
        .unwrap();
    let host_address = std::env::var("RUST_SERVICE_HOST").unwrap();
    HttpServer::new(|| {
        let cors = Cors::default()
            .allow_any_origin()
            .allowed_methods(vec!["POST"])
            .supports_credentials();

        App::new()
            .wrap_api()
            .wrap(Logger::new("%a %{User-Agent}i"))
            .wrap(cors)
            .service(web::scope("/api").wrap(SecretKey).service(execute))
            .with_json_spec_at("spec")
            .with_swagger_ui_at("/swagger-ui")
            .build()
    })
    .bind((host_address, port))?
    .run()
    .await
}
