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

pub const SOURCE_FILE_NAME: &str = "code";
#[cfg(windows)]
pub const COMPILED_FILE_NAME: &str = "compiled_file.exe";
#[cfg(not(windows))]
pub const COMPILED_FILE_NAME: &str = "compiled_file";
#[cfg(windows)]
pub const OS_PATH_PREFIX: &str = "";
#[cfg(not(windows))]
pub const OS_PATH_PREFIX: &str = "/usr/src/app/";

/// Решение пользователя
#[derive(Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct Solution {
    /// Выбранный язык
    /// Возможные варианты: rust, python, c, cpp, java, js
    lang: String,
    /// Исходный код решения
    source: String,
    /// Идентификатор пользователя
    uuid: String,
    /// Эталонные решения (только входные данные)
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
        format!("./{}_{}/", self.get_uuid(), self.get_hash())
    }
}

/// Статус выполнения запроса
#[derive(Serialize, JsonSchema)]
pub enum ExecuteStatus {
    /// Всё окей
    OK,
}

/// Информация о выполненном тесте
#[derive(Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ExecutedTest {
    /// Время выполнения в мс
    pub(crate) time: u128,
    /// Потребляемая память в Кб
    pub(crate) memory: u64,
    /// Поток вывода процесса
    pub(crate) result: String,
    /// Статус
    pub(crate) status: ExecuteStatus,
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
            std::fs::File::create(format!("{}/{}", folder, SOURCE_FILE_NAME)).unwrap();
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

/// Проверка решения пользователя
#[openapi(tag = "Compiling")]
#[post("/compile", format = "json", data = "<solution>")]
pub async fn compile(solution: Json<Solution>) -> status::Custom<Json<Vec<ExecutedTest>>> {
    let result = handle_solution(&solution).await;

    if let Ok(res) = result {
        status::Custom(Status::Ok, Json(res))
    } else {
        status::Custom(Status::BadRequest, Json(vec![]))
    }
}
