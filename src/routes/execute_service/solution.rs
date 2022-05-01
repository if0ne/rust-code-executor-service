use crate::routes::execute_service::CodeHasher;
use paperclip::actix::Apiv2Schema;
use serde::{Deserialize, Serialize};
use std::cell::Cell;
use std::fs::canonicalize;
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;
use std::path::PathBuf;

#[cfg(windows)]
pub const SPLITTER: &str = "&&&\r\n";
#[cfg(not(windows))]
pub const SPLITTER: &str = "&&&\n";

/// Решение пользователя
#[derive(Debug, Serialize, Deserialize, Apiv2Schema)]
#[serde(rename_all = "camelCase")]
pub struct Solution {
    /// Выбранный язык
    /// Возможные варианты: rust, python, c, cpp, java, js
    lang: String,
    /// Исходный код решения
    source: String,
    /// Идентификатор пользователя
    uuid: String,
    /// Время ожидания выполнения (в мс)
    timeout: u64,
    /// Эталонные решения (только входные данные)
    tests: Vec<String>,

    ///Кеш для хеша
    #[serde(skip)]
    cache_hash: Cell<Option<u64>>,
}

unsafe impl Send for Solution {}
unsafe impl Sync for Solution {}

impl Solution {
    /// Метод для создания объекта. Используется для тестирования
    pub fn new(lang: String, source: String, uuid: &str, timeout: u64, tests: Vec<String>) -> Self {
        Self {
            lang,
            source,
            uuid: uuid.to_string(),
            timeout,
            tests,
            cache_hash: Cell::new(None),
        }
    }

    /// UUID
    pub fn get_uuid(&self) -> &str {
        &self.uuid
    }

    /// Язык программирования
    pub fn get_lang(&self) -> &str {
        &self.lang
    }

    /// Список тестов
    pub fn get_tests(&self) -> &Vec<String> {
        &self.tests
    }

    /// Получение хеша по исходному коду
    fn get_hash<T: Hasher + Default>(&self, _: PhantomData<T>) -> u64 {
        if let Some(hash) = self.cache_hash.get() {
            hash
        } else {
            let mut hasher = T::default();
            self.source.hash(&mut hasher);
            let hash = hasher.finish();

            self.cache_hash.set(Some(hash));
            hash
        }
    }

    /// Исходный код
    pub fn get_src(&self) -> &str {
        &self.source
    }

    /// Название директории для работы с решением
    pub fn get_folder_name(&self) -> String {
        format!(
            "./{}_{}/",
            self.get_uuid(),
            self.get_hash(PhantomData::<CodeHasher>)
        )
    }

    #[allow(dead_code)]
    pub fn get_full_folder_path(&self) -> PathBuf {
        canonicalize(self.get_folder_name()).unwrap()
    }

    /// Время для таймаута в миллисекундах
    pub fn get_timeout_in_millis(&self) -> u64 {
        self.timeout
    }
}

/// Вспомогательная структура-билдер
pub struct SolutionBuilder {
    /// Выбранный язык
    lang: String,
    /// Исходный код решения
    source: String,
    /// Время ожидания выполнения (в мс)
    timeout: u64,
    /// Эталонные решения (только входные данные)
    tests: Vec<String>,
}

#[allow(dead_code)]
impl SolutionBuilder {
    /// Решение на Rust
    pub fn make_rust() -> Self {
        Self {
            lang: "rust".to_string(),
            source: "".to_string(),
            timeout: 0,
            tests: vec![],
        }
    }

    /// Решение на Python
    pub fn make_python() -> Self {
        Self {
            lang: "python".to_string(),
            source: "".to_string(),
            timeout: 0,
            tests: vec![],
        }
    }

    /// Решение на Java
    pub fn make_java() -> Self {
        Self {
            lang: "java".to_string(),
            source: "".to_string(),
            timeout: 0,
            tests: vec![],
        }
    }

    /// Решение на JavaScript
    pub fn make_js() -> Self {
        Self {
            lang: "js".to_string(),
            source: "".to_string(),
            timeout: 0,
            tests: vec![],
        }
    }

    /// Решение на C
    pub fn make_c() -> Self {
        Self {
            lang: "c".to_string(),
            source: "".to_string(),
            timeout: 0,
            tests: vec![],
        }
    }

    /// Решение на C++
    pub fn make_cpp() -> Self {
        Self {
            lang: "cpp".to_string(),
            source: "".to_string(),
            timeout: 0,
            tests: vec![],
        }
    }

    /// Решение на C#
    pub fn make_csharp() -> Self {
        Self {
            lang: "csharp".to_string(),
            source: "".to_string(),
            timeout: 0,
            tests: vec![],
        }
    }

    /// Решение на Kotlin
    pub fn make_kotlin() -> Self {
        Self {
            lang: "kotlin".to_string(),
            source: "".to_string(),
            timeout: 0,
            tests: vec![],
        }
    }

    /// Решение на Pascal
    pub fn make_pascal() -> Self {
        Self {
            lang: "pascal".to_string(),
            source: "".to_string(),
            timeout: 0,
            tests: vec![],
        }
    }

    /// Добавление исходного кода
    pub fn add_src(mut self, src: &str) -> Self {
        self.source = src.to_string();
        self
    }

    /// Загрузка исходного кода
    pub fn add_src_from_file<P: AsRef<std::path::Path>>(mut self, path: P) -> Self {
        use std::io::Read;

        let mut file =
            std::fs::File::open(path).unwrap(/*Не наша проблема, если тесты криво написаны*/);
        let mut src = String::new();
        file.read_to_string(&mut src).unwrap(/*Файлы с тестами как-то поломались*/);

        self.source = src;
        self
    }

    /// Установка таймаута
    pub fn add_timeout(mut self, timeout: u64) -> Self {
        self.timeout = timeout;
        self
    }

    /// Добавление одного теста
    pub fn add_test(mut self, test: &str) -> Self {
        self.tests.push(test.to_string());
        self
    }

    /// Добавление тестов
    pub fn add_tests(mut self, tests: &[&str]) -> Self {
        self.tests = tests.iter().map(|test| test.to_string()).collect();
        self
    }

    /// Загрузка тестов из файла
    pub fn add_tests_from_file<P: AsRef<std::path::Path>>(mut self, path: P) -> Self {
        use std::io::Read;

        let mut file =
            std::fs::File::open(path).unwrap(/*Не наша проблема, если тесты криво написаны*/);
        let mut tests = String::new();
        file.read_to_string(&mut tests).unwrap(/*Файлы с тестами как-то поломались*/);
        let tests = tests.split(SPLITTER).collect::<Vec<_>>();

        self.tests = tests.iter().map(|test| test.to_string()).collect();
        self
    }

    /// Сборка решения
    pub fn build(self) -> Solution {
        Solution::new(self.lang, self.source, "0000", self.timeout, self.tests)
    }
}
