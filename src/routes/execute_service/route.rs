use crate::executors::defined_language::DefinedLanguage;
use crate::executors::langs::java_exec::JavaExecutor;
use crate::executors::langs::python_exec::PythonExecutor;
use crate::executors::langs::rust_exec::RustExecutor;
use crate::routes::execute_service::executed_test::ExecutedTest;
use crate::routes::execute_service::solution::Solution;
use paperclip::actix::{
    api_v2_operation, post,
    web::{self},
};
use std::io::Write;
use std::path::Path;

/// Проверка решения пользователя
#[api_v2_operation]
#[post("/execute")]
pub async fn execute(solution: web::Json<Solution>) -> web::Json<Vec<ExecutedTest>> {
    let result = handle_solution(&solution).await;

    if let Ok(res) = result {
        web::Json(res)
    } else {
        web::Json(Vec::<ExecutedTest>::new())
    }
}

async fn handle_solution(solution: &Solution) -> Result<Vec<ExecutedTest>, ()> {
    let executor = define_lang(solution)?;

    create_exec_file(solution, &executor).await?;

    let executor = match executor {
        DefinedLanguage::Compiled(executor) => executor.compile(solution).await?,
        DefinedLanguage::Interpreted(executor) => executor.into(),
    };

    let results = solution
        .get_tests()
        .iter()
        .map(|test| executor.execute(solution, test))
        .collect::<Vec<_>>();

    let results = futures::future::join_all(results).await;

    executor.clean(solution).await;

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

async fn create_exec_file(solution: &Solution, executor: &DefinedLanguage) -> Result<(), ()> {
    let folder = solution.get_folder_name();
    if Path::new(&folder).exists() {
        return Err(());
    }

    {
        std::fs::create_dir(&folder).unwrap();
        let mut solution_file = std::fs::File::create(format!(
            "{}/{}",
            folder,
            executor.get_source_filename_with_ext(solution)
        ))
        .unwrap();
        solution_file
            .write_all(solution.get_src().as_bytes())
            .unwrap();
    }

    Ok(())
}
