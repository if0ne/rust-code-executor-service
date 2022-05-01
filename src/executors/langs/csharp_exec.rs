use crate::executors::consts::{COMPILED_FILE_NAME, OS_PATH_PREFIX};
use crate::executors::executor_impl::{ExecutorImpl, RunCommand};
use crate::make_compiler;
use crate::routes::execute_service::solution::Solution;

pub const COMPILER_NAME: &str = "dmcs";

#[cfg(windows)]
pub const EXECUTOR: Option<&str> = None;
#[cfg(not(windows))]
pub const EXECUTOR: Option<&str> = Some("mono");

pub struct CsharpExecutor;

impl ExecutorImpl for CsharpExecutor {
    fn get_compiler_args(&self, solution: &Solution) -> Result<Vec<String>, ()> {
        Ok(vec![
            COMPILER_NAME.to_string(),
            format!(
                "-out:{}{}/{}",
                OS_PATH_PREFIX,
                solution.get_folder_name(),
                COMPILED_FILE_NAME,
            ),
            format!(
                "{}{}/{}",
                OS_PATH_PREFIX,
                solution.get_folder_name(),
                self.get_source_filename_with_ext(solution)?
            ),
        ])
    }

    fn get_execute_args(&self, solution: &Solution) -> Result<(RunCommand, Vec<String>), ()> {
        Ok((
            EXECUTOR.map(|e| e.to_string()),
            vec![
                format!(
                    "{}{}",
                    solution.get_folder_name(),
                    COMPILED_FILE_NAME,
                ),
            ],
        ))
    }

    fn get_source_filename_with_ext(&self, solution: &Solution) -> Result<String, ()> {
        Ok(format!(
            "{}{}",
            self.get_source_filename(solution).unwrap(/*Всегда успешно*/),
            ".cs"
        ))
    }
}

make_compiler!(CsharpExecutor);

