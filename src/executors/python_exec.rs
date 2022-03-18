use crate::executors::ExecutorImpl;
use crate::make_interpreter;
use crate::routes::compile::{Solution, SOURCE_FILE_NAME};

pub struct PythonExecutor;

unsafe impl Sync for PythonExecutor {}
unsafe impl Send for PythonExecutor {}

impl ExecutorImpl for PythonExecutor {
    fn get_compiler_args(&self, _: &Solution) -> Vec<String> {
        panic!("Program invariant is broken")
    }

    fn get_execute_args(&self) -> (String, Vec<String>) {
        ("python".to_string(), vec![SOURCE_FILE_NAME.to_string()])
    }
}

make_interpreter!(PythonExecutor);
