use crate::executors::consts::{CONSOLE_ARG, CONSOLE_CALL};
use crate::executors::executor_impl::ExecutorImpl;
use crate::measure::ProcessInformer;
use crate::routes::execute_service::executed_test::ExecutedTest;
use crate::routes::execute_service::solution::Solution;
use std::io::Write;
use std::marker::PhantomData;

pub trait ExecutorState {}

pub struct Executor<S: ExecutorState> {
    inner: Box<dyn ExecutorImpl>,
    state: std::marker::PhantomData<S>,
}

pub struct Uncompiled;
pub struct Compiled;
pub struct Interpreted;

impl ExecutorState for Uncompiled {}
impl ExecutorState for Compiled {}
impl ExecutorState for Interpreted {}

unsafe impl Send for Executor<Uncompiled> {}
unsafe impl Sync for Executor<Uncompiled> {}

unsafe impl Send for Executor<Compiled> {}
unsafe impl Sync for Executor<Compiled> {}

unsafe impl Send for Executor<Interpreted> {}
unsafe impl Sync for Executor<Interpreted> {}

impl<S: ExecutorState> Executor<S> {
    pub fn new(exec: Box<dyn ExecutorImpl>) -> Self {
        Executor {
            inner: exec,
            state: PhantomData::<S>,
        }
    }

    pub fn get_source_filename_with_ext(&self, solution: &Solution) -> String {
        self.inner.get_source_filename_with_ext(solution)
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
            &mut process
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

        program_info.into()
    }

    pub async fn clean(self, solution: &Solution) {
        let folder = solution.get_folder_name();
        std::fs::remove_dir_all(&folder).unwrap();
    }
}
