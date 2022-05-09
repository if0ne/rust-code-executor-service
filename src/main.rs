#![warn(clippy::all)]
#![warn(rust_2018_idioms)]
#![allow(stable_features)]
#![feature(thread_is_running)]
mod executors;
mod measure;
mod models;
mod routes;
mod test;
mod utils;

use crate::routes::secret_key::SecretKey;
use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::web::Data;
use actix_web::{App, HttpServer};
use paperclip::actix::{web, OpenApiExt};
use routes::alive_service::route::alive;
use routes::execute_service::route::execute;

lazy_static::lazy_static! {
    static ref PORT: u16 = std::env::var("RUST_SERVICE_PORT").unwrap_or_else(|_| "8000".to_string()).parse().unwrap_or(8000);
    static ref HOST: String = std::env::var("RUST_SERVICE_HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    static ref THREAD_COUNT: std::num::NonZeroUsize = std::thread::available_parallelism().unwrap_or(std::num::NonZeroUsize::new(4).unwrap(/*Инвариант*/));
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));
    let _ = dotenv::dotenv();

    let _ = HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allowed_methods(vec!["GET"])
            .supports_credentials();

        let pool = Data::new(
            rayon::ThreadPoolBuilder::new()
                .num_threads(THREAD_COUNT.get())
                .build()
                .unwrap(/*Не ломается*/),
        );

        App::new()
            .wrap_api()
            .wrap(Logger::new("Endpoint: %r Code: %s Size: %b bytes Time: %D ms Date: %t Address: %a Browser: %{User-Agent}i"))
            .wrap(cors)
            .service(alive)
            .service(
                web::scope("/api")
                    .wrap(SecretKey)
                    .app_data(pool)
                    .service(execute),
            )
            .with_json_spec_at("spec")
            .with_swagger_ui_at("/swagger-ui")
            .build()
    })
    .bind((HOST.clone(), *PORT))?
    .run()
    .await;

    Ok(())
}
