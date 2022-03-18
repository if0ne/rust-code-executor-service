use crate::routes::compile::{ExecuteStatus, ExecutedTest, Solution};
use crate::ProcessInformer;
use std::io::Write;
use std::marker::PhantomData;
#[cfg(not(windows))]
use std::os::unix::fs::PermissionsExt;

pub mod python_exec;
pub mod rust_exec;

#[macro_use]
mod into_generator;

#[cfg(windows)]
pub const CONSOLE_CALL: &str = "cmd";
#[cfg(not(windows))]
pub const CONSOLE_CALL: &str = "sh";

#[cfg(windows)]
pub const CONSOLE_ARG: &str = "/C";
#[cfg(not(windows))]
pub const CONSOLE_ARG: &str = "-c";

pub enum DefinedLanguage {
    Compiled(Executor<Uncompiled>),
    Interpreted(Executor<Interpreted>),
}

pub struct Uncompiled;
pub struct Compiled;
pub struct Interpreted;

trait ExecutorImpl: Send + Sync {
    fn get_compiler_args(&self, solution: &Solution) -> Vec<String>;
    fn get_execute_args(&self) -> Vec<String>;
}

unsafe impl Send for Executor<Uncompiled> {}
unsafe impl Sync for Executor<Uncompiled> {}

unsafe impl Send for Executor<Compiled> {}
unsafe impl Sync for Executor<Compiled> {}

unsafe impl Send for Executor<Interpreted> {}
unsafe impl Sync for Executor<Interpreted> {}

impl From<Executor<Interpreted>> for Executor<Compiled> {
    fn from(exec: Executor<Interpreted>) -> Self {
        Executor {
            inner: exec.inner,
            state: PhantomData::<Compiled>,
        }
    }
}

impl Executor<Uncompiled> {
    pub async fn compile(self, solution: &Solution) -> Result<Executor<Compiled>, ()> {
        let compiler_args = self.inner.get_compiler_args(solution);

        let status = if cfg!(target_os = "windows") {
            std::process::Command::new(CONSOLE_CALL)
                .arg(CONSOLE_ARG)
                .args(compiler_args)
                .stdout(std::process::Stdio::piped())
                .stderr(std::process::Stdio::piped())
                .spawn()
                .unwrap()
                .wait()
                .unwrap()
        } else {
            std::process::Command::new(CONSOLE_CALL)
                .arg(CONSOLE_ARG)
                .arg(compiler_args.join(" "))
                .stdout(std::process::Stdio::piped())
                .stderr(std::process::Stdio::piped())
                .spawn()
                .unwrap()
                .wait()
                .unwrap()
        };

        if !status.success() {
            Err(())
        } else {
            Ok(Executor {
                inner: self.inner,
                state: PhantomData::<Compiled>,
            })
        }
    }
}

impl Executor<Compiled> {
    pub async fn execute(&self, solution: &Solution, test: &str) -> ExecutedTest {
        let folder = solution.get_folder_name();
        let execute_args = self.inner.get_execute_args().join("");
        let mut process = std::process::Command::new(folder.to_string() + &execute_args)
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()
            .unwrap();

        process
            .stdin
            .as_mut()
            .unwrap()
            .write_all(test.as_ref())
            .unwrap();
        let program_info = process.get_process_info().unwrap();

        ExecutedTest {
            time: program_info.execute_time.as_millis(),
            memory: program_info.total_memory / 1024,
            result: program_info.output,
            status: ExecuteStatus::OK,
        }
    }

    pub async fn clean(self, solution: &Solution) {
        let folder = solution.get_folder_name();
        std::fs::remove_dir_all(&folder).unwrap();
    }
}

pub struct Executor<S> {
    inner: Box<dyn ExecutorImpl>,
    state: std::marker::PhantomData<S>,
}
