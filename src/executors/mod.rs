use crate::routes::compile::{ExecuteStatus, ExecutedTest, Solution};
use crate::ProcessInformer;
use std::borrow::BorrowMut;
use std::io::Write;
use std::marker::PhantomData;
#[cfg(not(windows))]
use std::os::unix::fs::PermissionsExt;

pub mod java_exec;
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

impl DefinedLanguage {
    pub fn get_source_filename(&self, solution: &Solution) -> String {
        match self {
            DefinedLanguage::Compiled(exec) => exec.get_source_filename(solution),
            DefinedLanguage::Interpreted(exec) => exec.get_source_filename(solution),
        }
    }
}

pub struct Executor<S> {
    inner: Box<dyn ExecutorImpl>,
    state: std::marker::PhantomData<S>,
}

pub struct Uncompiled;
pub struct Compiled;
pub struct Interpreted;

type RunCommand = Option<String>;

trait ExecutorImpl {
    fn get_compiler_args(&self, solution: &Solution) -> Vec<String>;
    fn get_execute_args(&self, solution: &Solution) -> (RunCommand, Vec<String>);
    fn get_source_filename(&self, solution: &Solution) -> String;
}

unsafe impl Send for Executor<Uncompiled> {}
unsafe impl Sync for Executor<Uncompiled> {}

unsafe impl Send for Executor<Compiled> {}
unsafe impl Sync for Executor<Compiled> {}

unsafe impl Send for Executor<Interpreted> {}
unsafe impl Sync for Executor<Interpreted> {}

impl<T> Executor<T> {
    pub fn get_source_filename(&self, solution: &Solution) -> String {
        self.inner.get_source_filename(solution)
    }
}

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
        let (run_command, args) = self.inner.get_execute_args(solution);

        //TODO: две проверки одного и того же как-то не очень...
        let mut process = if let Some(ref run_command) = run_command {
            std::process::Command::new(run_command)
        } else {
            std::process::Command::new(args.join(""))
        };

        let process = if run_command.is_some() {
            process.args(args)
        } else {
            process.borrow_mut()
        };

        let mut process = process
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
