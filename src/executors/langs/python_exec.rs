use crate::executors::executor_impl::{ExecutorImpl, RunCommand};
use crate::make_interpreter;
use crate::routes::execute_service::solution::Solution;

pub struct PythonExecutor;

impl ExecutorImpl for PythonExecutor {
    fn get_compiler_args(&self, _: &Solution) -> Result<Vec<String>, ()> {
        panic!("Program invariant is broken")
    }

    fn get_execute_args(&self, solution: &Solution) -> Result<(RunCommand, Vec<String>), ()> {
        Ok((
            Some("python".to_string()),
            vec![format!(
                "{}{}",
                solution.get_folder_name(),
                self.get_source_filename_with_ext(solution)?
            )],
        ))
    }

    fn get_source_filename_with_ext(&self, solution: &Solution) -> Result<String, ()> {
        Ok(format!(
            "{}{}",
            self.get_source_filename(solution).unwrap(/*Всегда успешно*/),
            ".py"
        ))
    }
}

make_interpreter!(PythonExecutor);
