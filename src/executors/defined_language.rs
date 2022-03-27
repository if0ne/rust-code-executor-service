use crate::executors::executor::{Executor, Interpreted, Uncompiled};
use crate::routes::execute_service::solution::Solution;

pub enum DefinedLanguage {
    Compiled(Executor<Uncompiled>),
    Interpreted(Executor<Interpreted>),
}

impl DefinedLanguage {
    pub fn get_source_filename_with_ext(&self, solution: &Solution) -> String {
        match self {
            DefinedLanguage::Compiled(exec) => exec.get_source_filename_with_ext(solution),
            DefinedLanguage::Interpreted(exec) => exec.get_source_filename_with_ext(solution),
        }
    }
}
