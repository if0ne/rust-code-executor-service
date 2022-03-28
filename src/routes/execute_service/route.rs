use crate::executors::defined_language::DefinedLanguage;
use crate::executors::langs::java_exec::JavaExecutor;
use crate::executors::langs::python_exec::PythonExecutor;
use crate::executors::langs::rust_exec::RustExecutor;
use crate::routes::execute_service::executed_test::{
    ExecuteStatus, ExecutedResponse, ExecutedTest,
};
use crate::routes::execute_service::solution::Solution;
use paperclip::actix::{
    api_v2_operation, post,
    web::{self},
};
use std::io::Write;
use std::path::Path;

/// Проверка решения пользователя
#[api_v2_operation]
#[post("/execute", wrap = "SecretKey")]
pub async fn execute(solution: web::Json<Solution>) -> web::Json<ExecutedResponse> {
    let result = handle_solution(&solution).await;

    match result {
        Ok(result) => web::Json(ExecutedResponse::new(ExecuteStatus::OK, result)),
        Err(err) => web::Json(ExecutedResponse::new(err, vec![])),
    }
}

async fn handle_solution(solution: &Solution) -> Result<Vec<ExecutedTest>, ExecuteStatus> {
    let executor = define_lang(solution).map_err(|_| ExecuteStatus::UnsupportedLang)?;

    create_exec_file(solution, &executor).await?;

    let executor = match executor {
        DefinedLanguage::Compiled(executor) => executor
            .compile(solution)
            .await
            .map_err(|_| ExecuteStatus::CompileFail),
        DefinedLanguage::Interpreted(executor) => Ok(executor.into()),
    };

    if let Err(err) = executor {
        clean(solution).await;
        return Err(err);
    }

    let executor = executor.unwrap(/*invariant*/);

    let results = solution
        .get_tests()
        .iter()
        .map(|test| executor.execute(solution, test))
        .collect::<Vec<_>>();

    let results = futures::future::join_all(results).await;

    clean(solution).await;
    Ok(results)
}

fn define_lang(solution: &Solution) -> Result<DefinedLanguage, ()> {
    match solution.get_lang() {
        "rust" => Ok(RustExecutor.into()),
        "python" => Ok(PythonExecutor.into()),
        "java" => Ok(JavaExecutor.into()),
        _ => Err(()),
    }
}

async fn create_exec_file(
    solution: &Solution,
    executor: &DefinedLanguage,
) -> Result<(), ExecuteStatus> {
    let folder = solution.get_folder_name();
    if Path::new(&folder).exists() {
        return Err(ExecuteStatus::AlreadyTest);
    }

    {
        std::fs::create_dir(&folder).unwrap();
        let mut solution_file = std::fs::File::create(format!(
            "{}/{}",
            folder,
            executor.get_source_filename_with_ext(solution)
        ))
        .map_err(|_| ExecuteStatus::NoSpace)?;
        solution_file
            .write_all(solution.get_src().as_bytes())
            .map_err(|_| ExecuteStatus::IoFail)?;
    }

    Ok(())
}

pub async fn clean(solution: &Solution) {
    let folder = solution.get_folder_name();
    if std::fs::remove_dir_all(&folder).is_err() {
        log::warn!("Not found folder: {}", folder);
    }
}
