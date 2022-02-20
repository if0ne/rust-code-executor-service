mod mesure;

#[macro_use]
extern crate rocket;

use crate::mesure::ProcessInformer;

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
    {
        let mut solution_file = std::fs::File::create("test.txt").unwrap();
        solution_file.write_all(solution.code.as_bytes()).unwrap();
    }

    let mut process = std::process::Command::new("rustc")
        .arg("-O")
        .arg("test.txt")
        //.arg("--out-dir")
        //.arg("target/debug/")
        .spawn()
        .unwrap();
    let compile_info = process.get_process_info();

    //Вывод результата происходит в файл
    let outputs = std::fs::File::create("out.txt").unwrap();
    let mut process = std::process::Command::new("test.exe")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::from(outputs))
        .spawn()
        .unwrap();
    //Ввод данных в процесс
    process.stdin.as_mut().unwrap().write_all(b"Pavel").unwrap();
    let program_info = process.get_process_info();

    status::Custom(
        Status::Ok,
        format!("{:?}\n{:?}", compile_info, program_info),
    )
}

#[rocket::main]
async fn main() {
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
        .merge(("address", "0.0.0.0"))
        .merge(("port", 8000));

    rocket::custom(config)
        .attach(cors.to_cors().unwrap())
        .mount("/", routes![index, compile])
        .launch()
        .await
        .unwrap();
}
