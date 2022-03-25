use crate::executors::{ExecutorImpl, RunCommand};
use crate::make_compiler;
use crate::routes::compile::{Solution, COMPILED_FILE_NAME, OS_PATH_PREFIX, SOURCE_FILE_NAME};

pub struct RustExecutor;

impl ExecutorImpl for RustExecutor {
    fn get_compiler_args(&self, solution: &Solution) -> Vec<String> {
        vec![
            "rustc".to_string(),
            "-C".to_string(),
            "debuginfo=0".to_string(),
            "-C".to_string(),
            "opt-level=3".to_string(),
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
                SOURCE_FILE_NAME
            ),
        ]
    }

    fn get_execute_args(&self, solution: &Solution) -> (RunCommand, Vec<String>) {
        (
            None,
            vec![solution.get_folder_name() + &COMPILED_FILE_NAME.to_string()],
        )
    }

    fn get_source_filename(&self, _solution: &Solution) -> String {
        SOURCE_FILE_NAME.to_string()
    }
}

make_compiler!(RustExecutor);
