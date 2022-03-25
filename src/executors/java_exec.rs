use crate::executors::{ExecutorImpl, RunCommand};
use crate::make_compiler;
use crate::routes::compile::{Solution, OS_PATH_PREFIX};
use regex::Regex;

pub struct JavaExecutor;

impl ExecutorImpl for JavaExecutor {
    fn get_compiler_args(&self, solution: &Solution) -> Vec<String> {
        vec![
            "javac".to_string(),
            format!(
                "{}{}/{}",
                OS_PATH_PREFIX,
                solution.get_folder_name(),
                self.get_source_filename(solution)
            ),
        ]
    }

    fn get_execute_args(&self, solution: &Solution) -> (RunCommand, Vec<String>) {
        let file_name = self.get_source_filename(solution);
        let slice = &file_name[..file_name.len() - ".java".len()];
        (
            Some("java".to_string()),
            vec![
                "-classpath".to_string(),
                solution.get_folder_name(),
                slice.to_string(),
            ],
        )
    }

    fn get_source_filename(&self, solution: &Solution) -> String {
        let regex =
            Regex::new(r"public class (?P<class>.*) \{[\s\S]*public static void main").unwrap();
        let cap = regex.captures(solution.get_src()).unwrap();
        let s = cap[1].to_string() + ".java";
        s
    }
}

make_compiler!(JavaExecutor);
