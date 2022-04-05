use crate::executors::defined_language::DefinedLanguage;
use crate::executors::langs::c_exec::CExecutor;
use crate::executors::langs::cpp_exec::CppExecutor;
use crate::executors::langs::java_exec::JavaExecutor;
use crate::executors::langs::python_exec::PythonExecutor;
use crate::executors::langs::rust_exec::RustExecutor;
use crate::routes::execute_service::executed_test::{
    ExecuteStatus, ExecutedResponse, ExecutedTest,
};
use crate::routes::execute_service::solution::Solution;
use actix_web::web::Data;
use paperclip::actix::{
    api_v2_operation, post,
    web::{self},
};
use rayon::prelude::*;
use std::io::Write;
use std::path::Path;

/// Проверка решения пользователя
#[api_v2_operation]
#[post("execute", wrap = "SecretKey")]
pub async fn execute(
    solution: web::Json<Solution>,
    pool: Data<rayon::ThreadPool>,
) -> web::Json<ExecutedResponse> {
    let result = handle_solution(&solution, pool.get_ref()).await;

    match result {
        Ok(result) => web::Json(ExecutedResponse::new(
            ExecuteStatus::OK,
            result,
            "".to_string(),
        )),
        Err((err, stderr)) => web::Json(ExecutedResponse::new(err, vec![], stderr)),
    }
}

/// Обработка полученного запроса (/execute)
async fn handle_solution(
    solution: &Solution,
    pool: &rayon::ThreadPool,
) -> Result<Vec<ExecutedTest>, (ExecuteStatus, String)> {
    // Получение языка программирования
    let executor =
        define_lang(solution).map_err(|_| (ExecuteStatus::UnsupportedLang, "".to_string()))?;

    // Создание файла с кодом в зависимости от выбранного языка (см. Java)
    create_exec_file(solution, &executor)
        .await
        .map_err(|err| (err, "".to_string()))?;

    // Компиляция программы, если выбранный ЯП - то переход на стадию выполнения (Compiled)
    let executor = match executor {
        DefinedLanguage::Compiled(executor) => executor
            .compile(solution)
            .await
            .map_err(|err| (ExecuteStatus::CompileFail, err)),
        DefinedLanguage::Interpreted(executor) => Ok(executor.into()),
    };

    // Если произошла ошибка компиляции, то очищаем созданную директорию и возвращаем ошибку
    if let Err(err) = executor {
        clean(solution).await;
        return Err(err);
    }

    // Unwrap - инвариант, т.к. выше проверка на ошибку
    let executor = executor.unwrap();

    let results = pool.install(|| {
        solution
            .get_tests()
            .par_iter()
            .map(|test| executor.execute(solution, test))
            .collect::<Vec<ExecutedTest>>()
    });

    clean(solution).await;
    Ok(results)
}

/// Создание Executor-а в зависимости от отправленной строки
fn define_lang(solution: &Solution) -> Result<DefinedLanguage, ()> {
    match solution.get_lang() {
        "rust" => Ok(RustExecutor.into()),
        "python" => Ok(PythonExecutor.into()),
        "java" => Ok(JavaExecutor.into()),
        "c" => Ok(CExecutor.into()),
        "cpp" => Ok(CppExecutor.into()),
        _ => Err(()),
    }
}

/// Создание файла с кодом, отправленным пользователем
async fn create_exec_file(
    solution: &Solution,
    executor: &DefinedLanguage,
) -> Result<(), ExecuteStatus> {
    let folder = solution.get_folder_name();

    // Не делать проверку того же решение, если оно в настоящий момент проверяется
    if Path::new(&folder).exists() {
        return Err(ExecuteStatus::AlreadyTest);
    }

    let file_name = executor
        .get_source_filename_with_ext(solution)
        .map_err(|_| ExecuteStatus::CompileFail)?;

    std::fs::create_dir(&folder).unwrap();
    let mut solution_file = std::fs::File::create(format!("{}/{}", folder, file_name))
        .map_err(|_| ExecuteStatus::NoSpace)?;
    solution_file
        .write_all(solution.get_src().as_bytes())
        .map_err(|_| ExecuteStatus::IoFail)?;

    Ok(())
}

/// Очистка создаваемой папки с решением и executable-файлом
async fn clean(solution: &Solution) {
    let folder = solution.get_folder_name();
    if std::fs::remove_dir_all(&folder).is_err() {
        log::warn!("Can not remove a folder: {}", folder);
    }
}
