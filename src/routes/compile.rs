use crate::executors::python_exec::PythonExecutor;
use crate::executors::rust_exec::RustExecutor;
use crate::executors::DefinedLanguage;
use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;
use rocket_okapi::openapi;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::io::Write;
use std::path::Path;

pub const COMPILE_FILE_NAME: &str = "code";

#[derive(Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct Solution {
    lang: String,
    source: String,
    uuid: String,
    tests: Vec<String>,
}

unsafe impl Send for Solution {}
unsafe impl Sync for Solution {}

impl Solution {
    pub fn get_uuid(&self) -> String {
        self.uuid.clone()
    }

    //TODO: Настройка хешера
    pub fn get_hash(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.source.hash(&mut hasher);
        hasher.finish()
    }

    pub fn get_src(&self) -> &str {
        &self.source
    }

    pub fn get_folder_name(&self) -> String {
        format!("{}_{}", self.get_uuid(), self.get_hash())
    }
}

#[derive(Serialize, JsonSchema)]
pub enum ExecuteStats {
    OK,
}

#[derive(Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ExecutedTest {
    pub(crate) time: u128,
    pub(crate) memory: u64,
    pub(crate) result: String,
    pub(crate) status: ExecuteStats,
}

unsafe impl Send for ExecutedTest {}
unsafe impl Sync for ExecutedTest {}

fn define_lang(solution: &Solution) -> Result<DefinedLanguage, ()> {
    match solution.lang.as_str() {
        "rust" => Ok(RustExecutor.into()),
        "python" => Ok(PythonExecutor.into()),
        _ => Err(()),
    }
}

async fn create_exec_file(solution: &Solution) -> Result<(), ()> {
    let folder = solution.get_folder_name();
    if Path::new(&folder).exists() {
        return Err(());
    }

    {
        std::fs::create_dir(&folder).unwrap();
        let mut solution_file =
            std::fs::File::create(format!("{}/{}", folder, COMPILE_FILE_NAME)).unwrap();
        solution_file
            .write_all(solution.get_src().as_bytes())
            .unwrap();
    }

    Ok(())
}

async fn handle_solution(solution: &Solution) -> Result<Vec<ExecutedTest>, ()> {
    let executor = define_lang(solution)?;
    create_exec_file(solution).await?;
    let executor = match executor {
        DefinedLanguage::Compiled(executor) => executor.compile(solution).await?,
        DefinedLanguage::Interpreted(executor) => executor.into(),
    };

    let results = solution
        .tests
        .iter()
        .map(|test| executor.execute(solution, test))
        .collect::<Vec<_>>();

    let results = futures::future::join_all(results).await;

    executor.clean(solution).await;

    Ok(results)
}

#[openapi(tag = "compile")]
#[post("/compile", format = "json", data = "<solution>")]
pub async fn compile(solution: Json<Solution>) -> status::Custom<String> {
    let result = handle_solution(&solution).await;

    if let Ok(res) = result {
        status::Custom(Status::Ok, serde_json::to_string(&res).unwrap())
    } else {
        status::Custom(Status::BadRequest, String::new())
    }
}
