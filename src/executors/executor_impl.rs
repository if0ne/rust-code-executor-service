use crate::executors::consts::{COMPILED_FILE_NAME, SOURCE_FILE_NAME};
use crate::models::solution::Solution;

/// Команда запуска (для интерпретируемых языков)
pub type RunCommand = Option<String>;

pub trait ExecutorImpl: Send + Sync {
    /// Аргументы для компиляции
    fn get_compiler_args(&self, solution: &Solution) -> Result<Vec<String>, ()>;
    /// Аргументы для запуска теста
    fn get_execute_args(&self, solution: &Solution) -> Result<(RunCommand, Vec<String>), ()> {
        Ok((
            None,
            vec![solution.get_folder_name(), COMPILED_FILE_NAME.to_string()],
        ))
    }
    /// Название файла
    fn get_source_filename(&self, _: &Solution) -> Result<String, ()> {
        Ok(SOURCE_FILE_NAME.to_string())
    }
    /// Название файла с расширением
    fn get_source_filename_with_ext(&self, solution: &Solution) -> Result<String, ()>;
}
