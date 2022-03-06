use crate::executors::ExecutorImpl;
use crate::make_interpreter;
use crate::routes::compile::{Solution, COMPILE_FILE_NAME};

pub struct PythonExecutor;

unsafe impl Sync for PythonExecutor {}
unsafe impl Send for PythonExecutor {}

impl ExecutorImpl for PythonExecutor {
    fn get_compiler_args(&self, _: &Solution) -> Vec<String> {
        panic!("Program invariant is broken")
    }

    fn get_execute_args(&self) -> Vec<String> {
        vec!["python".to_string(), COMPILE_FILE_NAME.to_string()]
    }
}

make_interpreter!(PythonExecutor);
