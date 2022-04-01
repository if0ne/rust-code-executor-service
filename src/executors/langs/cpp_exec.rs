use crate::executors::consts::{COMPILED_FILE_NAME, OS_PATH_PREFIX, SOURCE_FILE_NAME};
use crate::executors::executor_impl::{ExecutorImpl, RunCommand};
use crate::make_compiler;
use crate::routes::execute_service::solution::Solution;

#[cfg(windows)]
pub const COMPILER_NAME: &str = "mingw32-g++";
#[cfg(not(windows))]
pub const COMPILER_NAME: &str = "gcc";

pub struct CppExecutor;

impl ExecutorImpl for CppExecutor {
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
            "-lstdc++".to_string(),
        ]
    }

    fn get_execute_args(&self, solution: &Solution) -> (RunCommand, Vec<String>) {
        (
            None,
            vec![solution.get_folder_name(), COMPILED_FILE_NAME.to_string()],
        )
    }

    fn get_source_filename(&self, _: &Solution) -> String {
        SOURCE_FILE_NAME.to_string()
    }

    fn get_source_filename_with_ext(&self, solution: &Solution) -> String {
        format!("{}{}", self.get_source_filename(solution), ".cpp")
    }
}

make_compiler!(CppExecutor);
