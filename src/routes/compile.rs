use crate::executors::rust_exec::RustExecutor;
use crate::executors::{Defined, Executor};
use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[derive(Deserialize)]
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

    pub fn get_hash(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.source.hash(&mut hasher);
        hasher.finish()
    }

    pub fn get_src(&self) -> &str {
        &self.source
    }
}

#[derive(Serialize)]
pub enum ExecuteStats {
    OK,
}

#[derive(Serialize)]
pub struct ExecutedTest {
    pub(crate) time: u64,
    pub(crate) memory: u64,
    pub(crate) result: String,
    pub(crate) status: ExecuteStats,
}

unsafe impl Send for ExecutedTest {}
unsafe impl Sync for ExecutedTest {}

fn define_lang(solution: &Solution) -> Result<Executor<Defined>, ()> {
    match solution.lang.as_str() {
        "rust" => Ok(RustExecutor {
            path: "".to_string(),
        }
        .into()),
        _ => Err(()),
    }
}

async fn handle_solution(solution: &Solution) -> Result<Vec<ExecutedTest>, ()> {
    let executor = define_lang(solution)?;
    let executor = executor.compile(solution).await?;

    let mut results = vec![];

    for item in solution.tests.iter() {
        results.push(executor.execute(item).await);
    }

    executor.clean().await;

    Ok(results)
}

#[post("/compile", format = "json", data = "<solution>")]
pub async fn compile(solution: Json<Solution>) -> status::Custom<String> {
    let result = handle_solution(&solution).await;

    if let Ok(res) = result {
        status::Custom(
            Status::Ok,
            serde_json::to_string(&res).unwrap(),
        )
    } else {
        status::Custom(
            Status::BadRequest,
            String::new(),
        )
    }
}
