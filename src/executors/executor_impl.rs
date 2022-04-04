use crate::executors::consts::SOURCE_FILE_NAME;
use crate::routes::execute_service::solution::Solution;

/// Команда запуска (для интерпретируемых языков)
pub type RunCommand = Option<String>;

pub trait ExecutorImpl {
    /// Аргументы для компиляции
    fn get_compiler_args(&self, solution: &Solution) -> Vec<String>;
    /// Аргументы для запуска теста
    fn get_execute_args(&self, solution: &Solution) -> (RunCommand, Vec<String>);
    /// Название файла
    //TODO: Сделать тип Result<String>, т.к. могут отправить Java-код, в котором нет класса
    fn get_source_filename(&self, _: &Solution) -> String {
        SOURCE_FILE_NAME.to_string()
    }
    /// Название файла с расширением
    fn get_source_filename_with_ext(&self, solution: &Solution) -> String;
}
