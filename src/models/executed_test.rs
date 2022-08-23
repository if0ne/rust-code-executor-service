use crate::measure::ProcessInfo;
use paperclip::actix::Apiv2Schema;
use serde::{Deserialize, Serialize};

/// Статус выполнения запроса
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Apiv2Schema)]
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
#[derive(Debug, Serialize, Deserialize, Apiv2Schema)]
#[serde(rename_all = "camelCase")]
pub struct ExecutedTest {
    /// Время выполнения в мс
    time: u128,
    /// Потребляемая память в Кб
    memory: u64,
    /// Поток вывода процесса
    stdout: String,
    /// Статус
    status: ExecuteStatus,
    /// Поток вывода ошибок во время выполнения
    stderr: String,
}

impl ExecutedTest {
    /// Для возвращение пустого элемента со статусом отличного от "OK"
    pub fn with_status(status: ExecuteStatus) -> Self {
        Self {
            time: 0,
            memory: 0,
            stdout: "".to_string(),
            status,
            stderr: "".to_string(),
        }
    }
}

/// Информация о выполненненых тестах
#[derive(Debug, Serialize, Deserialize, Apiv2Schema)]
#[serde(rename_all = "camelCase")]
pub struct ExecutedResponse {
    /// Статус
    status: ExecuteStatus,
    /// Все прошедшие тесты
    tests: Vec<ExecutedTest>,
    /// Поток вывода ошибок при компиляции
    stderr: String,
}

#[allow(dead_code)]
impl ExecutedResponse {
    /// Конструктор
    pub fn new(status: ExecuteStatus, tests: Vec<ExecutedTest>, stderr: String) -> Self {
        Self {
            status,
            tests,
            stderr,
        }
    }

    /// Получение статуса (для тестирования)
    pub fn get_status(&self) -> &ExecuteStatus {
        &self.status
    }

    /// Получение результатов выполнения программы
    pub fn get_raw_answers(&self) -> Vec<&str> {
        self.tests.iter().map(|test| test.stdout.as_str()).collect()
    }

    /// Среднее время выполнения теста
    pub fn get_avg_execute_time(&self) -> f64 {
        (self
            .tests
            .iter()
            .map(|e| e.time)
            .reduce(|accum, item| accum + item)
            .unwrap() as f64)
            / (self.tests.len() as f64)
    }

    /// Среднее количество затрачиваемой памяти
    pub fn get_avg_memory(&self) -> f64 {
        (self
            .tests
            .iter()
            .map(|e| e.memory)
            .reduce(|accum, item| accum + item)
            .unwrap() as f64)
            / (self.tests.len() as f64)
    }
}

impl From<ProcessInfo> for ExecutedTest {
    fn from(process_info: ProcessInfo) -> Self {
        // Если возвращаемое значение отличное от нуля, то ошибка произошла во время выполнения
        Self {
            time: process_info.execute_time.as_millis(),
            memory: process_info.total_memory / 1024, //Перевод в килобайты
            stdout: process_info.stdout,
            status: (process_info.exit_status == 0).then_some(ExecuteStatus::OK).unwrap_or(ExecuteStatus::RuntimeError),
            stderr: process_info.stderr,
        }
    }
}
