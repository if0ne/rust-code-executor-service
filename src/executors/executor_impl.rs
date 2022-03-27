use crate::executors::consts::SOURCE_FILE_NAME;
use crate::routes::execute_service::solution::Solution;

pub type RunCommand = Option<String>;

pub trait ExecutorImpl {
    fn get_compiler_args(&self, solution: &Solution) -> Vec<String>;
    fn get_execute_args(&self, solution: &Solution) -> (RunCommand, Vec<String>);
    //TODO: Сделать тип Result<String>, т.к. могут отправить Java-код, в котором нет класса
    fn get_source_filename(&self, _: &Solution) -> String {
        SOURCE_FILE_NAME.to_string()
    }
    fn get_source_filename_with_ext(&self, solution: &Solution) -> String;
}
