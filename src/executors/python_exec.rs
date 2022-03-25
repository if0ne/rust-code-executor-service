use crate::executors::{ExecutorImpl, RunCommand};
use crate::make_interpreter;
use crate::routes::compile::{Solution, SOURCE_FILE_NAME};

pub struct PythonExecutor;

impl ExecutorImpl for PythonExecutor {
    fn get_compiler_args(&self, _: &Solution) -> Vec<String> {
        panic!("Program invariant is broken")
    }

    fn get_execute_args(&self, solution: &Solution) -> (RunCommand, Vec<String>) {
        (
            Some("python3.9".to_string()),
            vec![solution.get_folder_name() + &SOURCE_FILE_NAME.to_string()],
        )
    }

    fn get_source_filename(&self, _solution: &Solution) -> String {
        SOURCE_FILE_NAME.to_string()
    }
}

make_interpreter!(PythonExecutor);
