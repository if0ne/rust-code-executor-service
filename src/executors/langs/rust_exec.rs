use crate::executors::consts::{COMPILED_FILE_NAME, OS_PATH_PREFIX};
use crate::executors::executor_impl::ExecutorImpl;
use crate::make_compiler;
use crate::routes::execute_service::solution::Solution;

pub struct RustExecutor;

impl ExecutorImpl for RustExecutor {
    fn get_compiler_args(&self, solution: &Solution) -> Result<Vec<String>, ()> {
        Ok(vec![
            "rustc".to_string(),
            "-C".to_string(),
            "debuginfo=0".to_string(),
            "-C".to_string(),
            "opt-level=3".to_string(),
            "-o".to_string(),
            format!(
                "{}{}{}",
                OS_PATH_PREFIX,
                solution.get_folder_name(),
                COMPILED_FILE_NAME
            ),
            format!(
                "{}{}{}",
                OS_PATH_PREFIX,
                solution.get_folder_name(),
                self.get_source_filename_with_ext(solution)?
            ),
        ])
    }

    fn get_source_filename_with_ext(&self, solution: &Solution) -> Result<String, ()> {
        Ok(format!(
            "{}{}",
            self.get_source_filename(solution).unwrap(/*Всегда успешно*/),
            ".rs"
        ))
    }
}

make_compiler!(RustExecutor);
