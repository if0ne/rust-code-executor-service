use crate::executors::consts::{COMPILED_FILE_NAME, OS_PATH_PREFIX};
use crate::executors::executor_impl::{ExecutorImpl, RunCommand};
use crate::make_compiler;
use crate::models::solution::Solution;

#[cfg(windows)]
pub const COMPILER_NAME: &str = "kotlinc";
#[cfg(not(windows))]
pub const COMPILER_NAME: &str = "/opt/kotlinc/bin/kotlinc";

pub struct KotlinExecutor;

impl ExecutorImpl for KotlinExecutor {
    fn get_compiler_args(&self, solution: &Solution) -> Result<Vec<String>, ()> {
        Ok(vec![
            COMPILER_NAME.to_string(),
            format!(
                "{}{}/{}",
                OS_PATH_PREFIX,
                solution.get_folder_name(),
                self.get_source_filename_with_ext(solution)?
            ),
            "-include-runtime".to_string(),
            "-d".to_string(),
            format!(
                "{}{}/{}",
                OS_PATH_PREFIX,
                solution.get_folder_name(),
                COMPILED_FILE_NAME.to_string() + ".jar",
            ),
        ])
    }

    fn get_execute_args(&self, solution: &Solution) -> Result<(RunCommand, Vec<String>), ()> {
        Ok((
            Some("java".to_string()),
            vec![
                "-jar".to_string(),
                format!(
                    "{}{}",
                    solution.get_folder_name(),
                    COMPILED_FILE_NAME.to_string() + ".jar",
                ),
            ],
        ))
    }

    fn get_source_filename_with_ext(&self, solution: &Solution) -> Result<String, ()> {
        Ok(format!(
            "{}{}",
            self.get_source_filename(solution).unwrap(/*Всегда успешно*/),
            ".kt"
        ))
    }
}

make_compiler!(KotlinExecutor);
