use crate::routes::compile::{ExecuteStats, ExecutedTest, Solution};
use crate::ProcessInformer;
use serde::de::Unexpected::Str;
use std::fmt::Debug;
use std::io::Write;
use std::marker::PhantomData;
#[cfg(not(windows))]
use std::os::unix::fs::PermissionsExt;

pub mod python_exec;
pub mod rust_exec;

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

#[async_trait::async_trait]
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

        let status = std::process::Command::new(CONSOLE_CALL)
            .arg(CONSOLE_ARG)
            .arg(compiler_args.join(" "))
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()
            .unwrap()
            .wait()
            .map_err(|_| ())?;

        /*  let mut solution_file =
            std::fs::File::open(format!("{}/{}", solution.get_folder_name(), "code")).unwrap();

        if !cfg!(windows) {
            let mut perm = solution_file.metadata().unwrap().permissions();
            println!("mode: {:o}", perm.mode());
            perm.set_mode(0o755);
            println!("mode: {:o}", perm.mode());
        }*/

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
        let execute_args = self.inner.get_execute_args();
        let mut process = std::process::Command::new(CONSOLE_CALL)
            .current_dir(&folder)
            .arg(CONSOLE_ARG)
            .arg(execute_args.join(""))
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
        let program_info = process.get_process_info().await.unwrap();

        ExecutedTest {
            time: program_info.execute_time.as_millis(),
            memory: program_info.total_memory / 1024,
            result: program_info.output,
            status: ExecuteStats::OK,
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
