use crate::executors::consts::{CONSOLE_ARG, CONSOLE_CALL};
use crate::executors::executor_impl::ExecutorImpl;
use crate::measure::ProcessInformer;
use crate::models::executed_test::{ExecuteStatus, ExecutedTest};
use crate::models::solution::Solution;
use crate::utils::read_from_buffer;
use std::io::{BufReader, Write};
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

unsafe impl<S: ExecutorState> Sync for Executor<S> {}
unsafe impl<S: ExecutorState> Send for Executor<S> {}

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
    pub fn get_source_filename_with_ext(&self, solution: &Solution) -> Result<String, ()> {
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
    pub async fn compile(self, solution: &Solution) -> Result<Executor<Compiled>, String> {
        let compiler_args = self
            .inner
            .get_compiler_args(solution)
            .map_err(|_| "".to_string())?;
        let (status, stderr) = compile_src_code(compiler_args).map_err(|_| "".to_string())?;

        if !status.success() {
            Err(stderr)
        } else {
            Ok(Executor {
                inner: self.inner,
                state: PhantomData::<Compiled>,
            })
        }
    }
}

/// Компиляция кода в Windows
#[allow(clippy::needless_question_mark)]
#[cfg(windows)]
fn compile_src_code(
    compiler_args: Vec<String>,
) -> Result<(std::process::ExitStatus, String), ExecuteStatus> {
    let mut compile_process = std::process::Command::new(CONSOLE_CALL)
        .arg(CONSOLE_ARG)
        .args(compiler_args)
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .map_err(|_| ExecuteStatus::CompileFail)?;
    let stderr = BufReader::new(compile_process.stderr.take().ok_or(ExecuteStatus::IoFail)?);
    let stderr = read_from_buffer(stderr)?;

    let status = compile_process
        .wait()
        .map_err(|_| ExecuteStatus::CompileFail)?;
    Ok((status, stderr))
}

/// Компиляция кода в Unix
#[allow(clippy::needless_question_mark)]
#[cfg(not(windows))]
fn compile_src_code(
    compiler_args: Vec<String>,
) -> Result<(std::process::ExitStatus, String), ExecuteStatus> {
    let mut compile_process = std::process::Command::new(CONSOLE_CALL)
        .arg(CONSOLE_ARG)
        .arg(compiler_args.join(" "))
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .map_err(|_| ExecuteStatus::CompileFail)?;

    let stderr = BufReader::new(compile_process.stderr.take().ok_or(ExecuteStatus::IoFail)?);
    let stderr = read_from_buffer(stderr)?;

    let status = compile_process
        .wait()
        .map_err(|_| ExecuteStatus::CompileFail)?;
    Ok((status, stderr))
}

impl Executor<Compiled> {
    /// Выполнение теста
    pub fn execute(&self, solution: &Solution, test: &str) -> ExecutedTest {
        //TODO: Возможно никогда не упадет
        let (run_command, args) = self.inner.get_execute_args(solution).unwrap();

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

            match process.get_process_info(std::time::Duration::from_millis(
                solution.get_timeout_in_millis(),
            )) {
                Ok(program_info) => program_info.into(),
                Err(err) => ExecutedTest::with_status(err),
            }
        } else {
            ExecutedTest::with_status(ExecuteStatus::RuntimeError)
        }
    }
}
