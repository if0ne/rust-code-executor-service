#![warn(clippy::all)]
#![warn(rust_2018_idioms)]
mod mesure;

#[macro_use]
extern crate rocket;

use crate::mesure::ProcessInformer;
use dotenv::dotenv;
use std::env;

use rocket::http::{Method, Status};
use rocket::response::status;
use rocket::serde::json::Json;
use rocket::serde::Deserialize;
use rocket::Config;
use rocket_cors::{AllowedOrigins, CorsOptions};

use std::io::Write;

#[derive(Deserialize)]
struct Solution {
    code: String,
}

#[get("/")]
async fn index() -> status::Custom<String> {
    status::Custom(Status::Ok, String::from("All is ok or not"))
}

#[post("/compile", format = "json", data = "<solution>")]
async fn compile(solution: Json<Solution>) -> status::Custom<String> {
    let executable_file_path = if cfg!(target_os = "windows") {
        "./target/test.exe"
    } else {
        "./target/test"
        //для java ./target/AppRunner (соответсвует названию класса)
    };

    {
        let mut solution_file = std::fs::File::create("test.txt").unwrap();
        solution_file.write_all(solution.code.as_bytes()).unwrap();
    }

    let mut process = std::process::Command::new("rustc")
        .arg("-O")
        //.arg("-d") для java
        //.arg("target")
        //.arg("test.java")
        .arg("test.txt")
        .arg("--out-dir")
        .arg("target")
        .spawn()
        .unwrap();
    let compile_info = process.get_process_info();

    let mut process = std::process::Command::new(executable_file_path)
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .unwrap();
    //Ввод данных в процесс
    process.stdin.as_mut().unwrap().write_all(b"Pavel").unwrap();
    let program_info = process.get_process_info();
    let output = process.wait_with_output().unwrap();

    std::fs::remove_file("test.txt").unwrap();

    status::Custom(
        Status::Ok,
        format!(
            "{:?}\n{:?}\n{}",
            compile_info,
            program_info,
            String::from_utf8_lossy(&output.stdout)
        ),
    )
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
