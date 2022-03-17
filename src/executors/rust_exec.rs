use crate::executors::{DefinedLanguage, Executor, ExecutorImpl, Uncompiled};
use crate::routes::compile::{Solution, COMPILED_FILE_NAME, COMPILE_FILE_NAME, OS_PATH_PREFIX};
use std::marker::PhantomData;

pub struct RustExecutor;

unsafe impl Sync for RustExecutor {}
unsafe impl Send for RustExecutor {}

impl ExecutorImpl for RustExecutor {
    fn get_compiler_args(&self, solution: &Solution) -> Vec<String> {
        vec![
            "rustc".to_string(),
            "-C".to_string(),
            "debuginfo=0".to_string(),
            "-C".to_string(),
            "opt-level=3".to_string(),
            "-o".to_string(),
            format!(
                "{}{}/{}",
                OS_PATH_PREFIX,
                solution.get_folder_name(),
                COMPILED_FILE_NAME
            ),
            format!(
                "{}{}/{}",
                OS_PATH_PREFIX,
                solution.get_folder_name(),
                COMPILE_FILE_NAME
            ),
        ]
    }

    fn get_execute_args(&self) -> Vec<String> {
        if !cfg!(target_os = "linux") {
            vec![COMPILED_FILE_NAME.to_string()]
        } else {
            vec![COMPILED_FILE_NAME.to_string()]
        }
    }
}

impl From<RustExecutor> for DefinedLanguage {
    fn from(exec: RustExecutor) -> Self {
        DefinedLanguage::Compiled(Executor {
            inner: Box::new(exec),
            state: PhantomData::<Uncompiled>,
        })
    }
}
