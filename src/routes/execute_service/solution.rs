use paperclip::actix::Apiv2Schema;
use serde::Deserialize;
use std::cell::Cell;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

/// Решение пользователя
#[derive(Deserialize, Apiv2Schema)]
#[serde(rename_all = "camelCase")]
pub struct Solution {
    /// Выбранный язык
    /// Возможные варианты: rust, python, c, cpp, java, js
    lang: String,
    /// Исходный код решения
    source: String,
    /// Идентификатор пользователя
    uuid: String,
    /// Эталонные решения (только входные данные)
    tests: Vec<String>,

    ///Кеш для хеша
    #[serde(skip)]
    cache_hash: Cell<Option<u64>>,
}

unsafe impl Send for Solution {}
unsafe impl Sync for Solution {}

impl Solution {
    pub fn get_uuid(&self) -> &str {
        &self.uuid
    }

    pub fn get_lang(&self) -> &str {
        &self.lang
    }

    pub fn get_tests(&self) -> &Vec<String> {
        &self.tests
    }

    //TODO: Настройка хешера
    fn get_hash(&self) -> u64 {
        if let Some(hash) = self.cache_hash.get() {
            hash
        } else {
            let mut hasher = DefaultHasher::new();
            self.source.hash(&mut hasher);
            let hash = hasher.finish();

            self.cache_hash.set(Some(hash));
            hash
        }
    }

    pub fn get_src(&self) -> &str {
        &self.source
    }

    pub fn get_folder_name(&self) -> String {
        format!("./{}_{}/", self.get_uuid(), self.get_hash())
    }
}
