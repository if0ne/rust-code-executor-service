use crate::measure::ProcessInfo;
use paperclip::actix::Apiv2Schema;
use serde::Serialize;

/// Статус выполнения запроса
#[derive(Serialize, Apiv2Schema)]
pub enum ExecuteStatus {
    /// Всё окей
    OK,
}

/// Информация о выполненном тесте
#[derive(Serialize, Apiv2Schema)]
#[serde(rename_all = "camelCase")]
pub struct ExecutedTest {
    /// Время выполнения в мс
    time: u128,
    /// Потребляемая память в Кб
    memory: u64,
    /// Поток вывода процесса
    result: String,
    /// Статус
    status: ExecuteStatus,
}

impl From<ProcessInfo> for ExecutedTest {
    fn from(process_info: ProcessInfo) -> Self {
        Self {
            time: process_info.execute_time.as_millis(),
            memory: process_info.total_memory / 1024,
            result: process_info.output,
            //TODO: Сделать правильный ExecuteStatus
            status: ExecuteStatus::OK,
        }
    }
}
