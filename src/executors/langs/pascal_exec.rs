use crate::executors::consts::{OS_PATH_PREFIX};
use crate::executors::executor_impl::{ExecutorImpl, RunCommand};
use crate::make_compiler;
use crate::routes::execute_service::solution::Solution;

#[cfg(windows)]
pub const COMPILER_NAME: &str = "pabcnetcclear";
#[cfg(not(windows))]
pub const COMPILER_NAME: &str = "mono /opt/pabcnetc/pabcnetcclear.exe";

pub struct PascalExecutor;

impl ExecutorImpl for PascalExecutor {
    fn get_compiler_args(&self, solution: &Solution) -> Result<Vec<String>, ()> {
        Ok(vec![
            COMPILER_NAME.to_string(),
            format!(
                "{}{}/{}",
                OS_PATH_PREFIX,
                solution.get_folder_name(),
                self.get_source_filename_with_ext(solution)?
            ),
        ])
    }

    fn get_execute_args(&self, solution: &Solution) -> Result<(RunCommand, Vec<String>), ()> {
        Ok((
            Some("mono".to_string()),
            vec![
                solution.get_folder_name(),
                self.get_source_filename(solution).unwrap(),
            ],
        ))
    }

    fn get_source_filename_with_ext(&self, solution: &Solution) -> Result<String, ()> {
        Ok(format!(
            "{}{}",
            self.get_source_filename(solution).unwrap(/*Всегда успешно*/),
            ".pas"
        ))
    }
}

make_compiler!(PascalExecutor);
