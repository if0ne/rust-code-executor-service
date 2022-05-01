use crate::executors::consts::OS_PATH_PREFIX;
use crate::executors::executor_impl::{ExecutorImpl, RunCommand};
use crate::make_compiler;
use crate::routes::execute_service::solution::Solution;
use regex::Regex;

pub struct JavaExecutor;

impl ExecutorImpl for JavaExecutor {
    fn get_compiler_args(&self, solution: &Solution) -> Result<Vec<String>, ()> {
        Ok(vec![
            "javac".to_string(),
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
            Some("java".to_string()),
            vec![
                "-classpath".to_string(),
                solution.get_folder_name(),
                self.get_source_filename(solution)?,
            ],
        ))
    }

    fn get_source_filename(&self, solution: &Solution) -> Result<String, ()> {
        let regex = Regex::new(r"public class (?P<class>.*) \{[\s\S]*public static void main").unwrap(/*Регулярка правильная*/);
        let capture = regex.captures(solution.get_src()).ok_or(())?;
        Ok(capture.name("class").map(|m| m.as_str()).ok_or(())?.trim().to_string())
    }

    fn get_source_filename_with_ext(&self, solution: &Solution) -> Result<String, ()> {
        Ok(format!(
            "{}{}",
            self.get_source_filename(solution)?,
            ".java"
        ))
    }
}

make_compiler!(JavaExecutor);

#[allow(unused_imports)]
mod tests {
    use crate::executors::executor_impl::ExecutorImpl;
    use crate::executors::langs::java_exec::JavaExecutor;
    use crate::routes::execute_service::solution::SolutionBuilder;

    #[test]
    fn parse_class_name() {
        let java_exec = JavaExecutor;
        let sol = SolutionBuilder::make_java()
            .add_src("import java.util.Scanner;\n\npublic class Main {\n\tpublic static void main(String[] args) {\n\t    Scanner scanner = new Scanner(System.in);\n\t    long a = scanner.nextInt();\n\t    long b = scanner.nextInt();\n\t\tSystem.out.println(a + b);\n\t}\n}")
            .build();


        assert_eq!(java_exec.get_source_filename(&sol), Ok("Main".to_string()));
    }
}