use crate::executors::ExecutorImpl;
use crate::make_compiler;
use crate::routes::compile::{Solution, COMPILE_FILE_NAME};

pub struct RustExecutor;

unsafe impl Sync for RustExecutor {}
unsafe impl Send for RustExecutor {}

impl ExecutorImpl for RustExecutor {
    fn get_compiler_args(&self, solution: &Solution) -> Vec<String> {
        vec![
            "rustc".to_string(),
            "-C".to_string(),
            "debuginfo=0".to_string(),
            "-C".to_string(),
            "opt-level=3".to_string(),
            "-O".to_string(),
            format!("{}/{}", solution.get_folder_name(), COMPILE_FILE_NAME),
            "--out-dir".to_string(),
            solution.get_folder_name(),
        ]
    }

    fn get_execute_args(&self) -> Vec<String> {
        vec![COMPILE_FILE_NAME.to_string()]
    }
}

make_compiler!(RustExecutor);
