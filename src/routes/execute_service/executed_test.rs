use crate::measure::ProcessInfo;
use paperclip::actix::Apiv2Schema;
use serde::Serialize;

/// Статус выполнения запроса
#[derive(Debug, Serialize, Apiv2Schema)]
pub enum ExecuteStatus {
    /// Всё окей
    OK,
    /// Решение было отправлено до завершения такого же решения
    AlreadyTest,
    /// Закончилось место на сервере
    NoSpace,
    /// Ошибка во время компиляции
    CompileFail,
    /// Ошибка во время выполнения
    RuntimeError,
    /// Неподдерживаемый язык
    UnsupportedLang,
    /// Проблема с вводом/выводом в процесс
    IoFail,
    /// Процесс работает слишком долго, либо зациклился
    Timeout,
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

impl ExecutedTest {
    pub fn with_status(status: ExecuteStatus) -> Self {
        Self {
            time: 0,
            memory: 0,
            result: "".to_string(),
            status,
        }
    }
}

/// Информация о выполненном тесте
#[derive(Serialize, Apiv2Schema)]
#[serde(rename_all = "camelCase")]
pub struct ExecutedResponse {
    /// Статус
    status: ExecuteStatus,
    /// Все прошедшие тесты
    tests: Vec<ExecutedTest>,
}

impl ExecutedResponse {
    pub fn new(status: ExecuteStatus, tests: Vec<ExecutedTest>) -> Self {
        Self { status, tests }
    }
}

impl From<ProcessInfo> for ExecutedTest {
    fn from(process_info: ProcessInfo) -> Self {
        Self {
            time: process_info.execute_time.as_millis(),
            memory: process_info.total_memory / 1024,
            result: process_info.output,
            status: if process_info.exit_status == 0 {
                ExecuteStatus::OK
            } else {
                ExecuteStatus::RuntimeError
            },
        }
    }
}
