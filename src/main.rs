mod mesure;

#[macro_use]
extern crate rocket;

use crate::mesure::ProcessInformer;
use rocket::http::{Method, Status};
use rocket::response::status;
use std::io::Write;
use rocket_cors::{AllowedOrigins, CorsOptions};

#[get("/")]
async fn index() -> status::Custom<String> {
    let mut process = std::process::Command::new("rustc")
        .arg("-O")
        .arg("test.txt")
        .arg("--out-dir")
        .arg("target/debug/")
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
    rocket::build()
        .attach(cors.to_cors().unwrap())
        .mount("/", routes![index])
        .launch()
        .await
        .unwrap();
}
