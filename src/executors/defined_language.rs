use crate::executors::executor::{Executor, Interpreted, Uncompiled};
use crate::routes::execute_service::solution::Solution;

/// Определенный язык программирования
pub enum DefinedLanguage {
    /// Компилируемый
    Compiled(Executor<Uncompiled>),
    /// Интерпретируемый
    Interpreted(Executor<Interpreted>),
}

impl DefinedLanguage {
    /// Название исходного файла с кодом
    pub fn get_source_filename_with_ext(&self, solution: &Solution) -> Result<String, ()> {
        match self {
            DefinedLanguage::Compiled(exec) => exec.get_source_filename_with_ext(solution),
            DefinedLanguage::Interpreted(exec) => exec.get_source_filename_with_ext(solution),
        }
    }
}
