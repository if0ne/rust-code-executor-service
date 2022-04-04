use crate::executors::consts::{CONSOLE_ARG, CONSOLE_CALL};
use crate::executors::executor_impl::ExecutorImpl;
use crate::measure::ProcessInformer;
use crate::routes::execute_service::executed_test::{ExecuteStatus, ExecutedTest};
use crate::routes::execute_service::solution::Solution;
use std::io::Write;
use std::marker::PhantomData;

/// Трейт-марка для одозначения состояние экзекьютора
pub trait ExecutorState {}

/// Экзекьютор - интерфейс
pub struct Executor<S: ExecutorState> {
    /// Внутренний экзекьютор (язык программирования)
    inner: Box<dyn ExecutorImpl>,
    /// Состояние экзекьютора
    state: std::marker::PhantomData<S>,
}

/// Нескомпилированное состояние
pub struct Uncompiled;
/// Состояние, в котором можно выполнять тесты
pub struct Compiled;
/// Обозначение интерпретируемого языка программирования
pub struct Interpreted;

impl ExecutorState for Uncompiled {}
impl ExecutorState for Compiled {}
impl ExecutorState for Interpreted {}

impl<S: ExecutorState> Executor<S> {
    pub fn new(exec: Box<dyn ExecutorImpl>) -> Self {
        Executor {
            inner: exec,
            state: PhantomData::<S>,
        }
    }

    /// Название исходного файла с кодом
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
    /// Компиляция исходного кода
    pub async fn compile(self, solution: &Solution) -> Result<Executor<Compiled>, ()> {
        let compiler_args = self.inner.get_compiler_args(solution);
        let status = compile_src_code(compiler_args)?;

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

/// Компиляция кода в Windows
#[cfg(windows)]
fn compile_src_code(compiler_args: Vec<String>) -> Result<std::process::ExitStatus, ()> {
    Ok(std::process::Command::new(CONSOLE_CALL)
        .arg(CONSOLE_ARG)
        .args(compiler_args)
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .map_err(|_| ())?
        .wait()
        .map_err(|_| ())?)
}

/// Компиляция кода в Unix
#[cfg(not(windows))]
fn compile_src_code(compiler_args: Vec<String>) -> Result<std::process::ExitStatus, ()> {
    Ok(std::process::Command::new(CONSOLE_CALL)
        .arg(CONSOLE_ARG)
        .arg(compiler_args.join(" "))
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .map_err(|_| ())?
        .wait()
        .map_err(|_| ())?)
}

impl Executor<Compiled> {
    /// Выполнение теста
    pub async fn execute(&self, solution: &Solution, test: &str) -> ExecutedTest {
        let (run_command, args) = self.inner.get_execute_args(solution);

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

        let process = process
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn();

        if let Ok(mut process) = process {
            if let Some(stdin) = process.stdin.as_mut() {
                if stdin.write_all(test.as_ref()).is_err() {
                    return ExecutedTest::with_status(ExecuteStatus::IoFail);
                }
            } else {
                return ExecutedTest::with_status(ExecuteStatus::IoFail);
            }

            match process
                .get_process_info(std::time::Duration::new(0, solution.get_timeout_in_nano()))
            {
                Ok(program_info) => program_info.into(),
                Err(err) => ExecutedTest::with_status(err),
            }
        } else {
            ExecutedTest::with_status(ExecuteStatus::RuntimeError)
        }
    }
}
