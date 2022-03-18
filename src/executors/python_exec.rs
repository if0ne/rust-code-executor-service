use crate::executors::{DefinedLanguage, Executor, ExecutorImpl, Interpreted};
use crate::routes::compile::{Solution, COMPILE_FILE_NAME};
use std::marker::PhantomData;

pub struct PythonExecutor;

unsafe impl Sync for PythonExecutor {}
unsafe impl Send for PythonExecutor {}

impl ExecutorImpl for PythonExecutor {
    fn get_compiler_args(&self, _: &Solution) -> Vec<String> {
        panic!("Program invariant is broken")
    }

    fn get_execute_args(&self) -> Vec<String> {
        vec!["python".to_string(), COMPILE_FILE_NAME.to_string()]
    }
}

impl From<PythonExecutor> for DefinedLanguage {
    fn from(exec: PythonExecutor) -> Self {
        DefinedLanguage::Interpreted(Executor {
            inner: Box::new(exec),
            state: PhantomData::<Interpreted>,
        })
    }
}