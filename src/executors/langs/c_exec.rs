use crate::executors::consts::{OS_PATH_PREFIX,COMPILED_FILE_NAME,SOURCE_FILE_NAME};
use crate::executors::executor_impl::{ExecutorImpl, RunCommand};
use crate::make_compiler;
use crate::routes::execute_service::solution::Solution;
use regex::Regex;

#[cfg(windows)]
pub const COMPILER_NAME: &str = "mingw32-gcc";
#[cfg(not(windows))]
pub const COMPILER_NAME: &str = "gcc";

pub struct CExecutor;

impl ExecutorImpl for CExecutor {
    fn get_compiler_args(&self, solution: &Solution) -> Vec<String> {
        vec![
            COMPILER_NAME.to_string(),
            "-O3".to_string(),
            "-o".to_string(),
            format!(
                "{}{}/{}",
                OS_PATH_PREFIX,
                solution.get_folder_name(),
                COMPILED_FILE_NAME
            ),
            format!(
                "{}{}/{}",
                OS_PATH_PREFIX,
                solution.get_folder_name(),
                self.get_source_filename_with_ext(solution)
            ),
        ]
    }

    fn get_execute_args(&self, solution: &Solution) -> (RunCommand, Vec<String>) {
        (
            None,
            vec![
                solution.get_folder_name(),
                COMPILED_FILE_NAME.to_string(),
            ],
        )
    }

    fn get_source_filename(&self, solution: &Solution) -> String {
        SOURCE_FILE_NAME.to_string()
    }

    fn get_source_filename_with_ext(&self, solution: &Solution) -> String {
        format!("{}{}", self.get_source_filename(solution), ".c")
    }
}

make_compiler!(CExecutor);
