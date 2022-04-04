use crate::routes::execute_service::executed_test::ExecuteStatus;
use std::time::Duration;

/// Информация о запускаемом процессе
#[derive(Debug)]
pub struct ProcessInfo {
    /// Время выполнения
    pub execute_time: Duration,
    /// Пик занимаемой оперативной памяти
    pub total_memory: u64,
    /// Вывод из стандартного потока вывода
    pub output: String,
    /// Возвращаемое значение процесса (если 0 - то, всё ок, иначе всё плохо)
    pub exit_status: i32,
}

/// Для расширения функциональности std::process::Child
pub trait ProcessInformer {
    fn get_process_info(self, timeout: std::time::Duration) -> Result<ProcessInfo, ExecuteStatus>;
}

pub mod info;
