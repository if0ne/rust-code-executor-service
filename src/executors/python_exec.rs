use crate::executors::{ExecutorImpl, RunCommand};
use crate::make_interpreter;
use crate::routes::compile::{Solution};

pub struct PythonExecutor;

impl ExecutorImpl for PythonExecutor {
    fn get_compiler_args(&self, _: &Solution) -> Vec<String> {
        panic!("Program invariant is broken")
    }

    fn get_execute_args(&self, solution: &Solution) -> (RunCommand, Vec<String>) {
        (
            Some("python3.9".to_string()),
            vec![format!(
                "{}{}",
                solution.get_folder_name(),
                self.get_source_filename_with_ext(solution)
            )],
        )
    }

    fn get_source_filename_with_ext(&self, solution: &Solution) -> String {
        format!("{}{}", self.get_source_filename(solution), ".py")
    }
}

make_interpreter!(PythonExecutor);
