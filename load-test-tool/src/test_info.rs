use std::collections::HashMap;

use iced::{Column, Element, Text};

use crate::LoadTestMessage;

#[derive(Debug, Clone)]
pub struct TestInfo {
    lang_count: HashMap<String, u32>,
    create_time: std::time::Duration,
    test_time: std::time::Duration,
    delay: u64,
    reqs_count: u32,
    execute_time: f64,
    memory: f64,
}

impl TestInfo {
    pub fn new(reqs: u32) -> Self {
        TestInfo {
            lang_count: {
                let mut langs = HashMap::new();
                langs.insert("c".to_string(), 0);
                langs.insert("cpp".to_string(), 0);
                langs.insert("csharp".to_string(), 0);
                langs.insert("java".to_string(), 0);
                langs.insert("kotlin".to_string(), 0);
                langs.insert("rust".to_string(), 0);
                langs.insert("js".to_string(), 0);
                langs.insert("python".to_string(), 0);
                langs.insert("pascal".to_string(), 0);

                langs
            },
            create_time: std::time::Duration::new(0, 0),
            test_time: std::time::Duration::new(0, 0),
            delay: 0,
            reqs_count: reqs,
            execute_time: 0.0,
            memory: 0.0,
        }
    }

    pub fn add_lang_req(&mut self, lang: &str) {
        let lang_count = self.lang_count.get_mut(lang).unwrap(/*Инвариант*/);
        *lang_count += 1;
    }

    pub fn add_create_time(&mut self, time: std::time::Duration) {
        self.create_time = time;
    }

    pub fn add_test_time(&mut self, time: std::time::Duration) {
        self.test_time = time;
    }

    pub fn add_generated_delay(&mut self, delay: u64) {
        self.delay += delay;
    }

    pub fn add_execute_time(&mut self, time: f64) {
        self.execute_time += time;
    }

    pub fn add_memory(&mut self, memory: f64) {
        self.memory += memory;
    }

    pub fn get_info(&self) -> Element<LoadTestMessage> {
        Column::new()
            .spacing(10)
            .align_items(iced::Alignment::Start)
            .push(Text::new("Stats").size(40))
            .push(Text::new(format!("Request count: {}", self.reqs_count)))
            .push(
                Column::new()
                    .spacing(5)
                    .push(Text::new(format!("C: {}", self.lang_count["c"])))
                    .push(Text::new(format!("C++: {}", self.lang_count["cpp"])))
                    .push(Text::new(format!("C#: {}", self.lang_count["csharp"])))
                    .push(Text::new(format!("Java: {}", self.lang_count["java"])))
                    .push(Text::new(format!("Kotlin: {}", self.lang_count["kotlin"])))
                    .push(Text::new(format!("Rust: {}", self.lang_count["rust"])))
                    .push(Text::new(format!("JavaScript: {}", self.lang_count["js"])))
                    .push(Text::new(format!("Python: {}", self.lang_count["python"])))
                    .push(Text::new(format!("Pascal: {}", self.lang_count["pascal"]))),
            )
            .push(
                Column::new()
                    .spacing(5)
                    .push(Text::new(format!(
                        "Request time creation: {} ms",
                        self.create_time.as_millis()
                    )))
                    .push(Text::new(format!(
                        "All time: {} ms",
                        self.test_time.as_millis()
                    ))),
            )
            .push(
                Column::new()
                    .spacing(5)
                    .push(Text::new(format!(
                        "Avg delay: {} ms",
                        (self.delay as f64) / (self.reqs_count as f64)
                    )))
                    .push(Text::new(format!(
                        "Avg execute time: {} ms",
                        (self.execute_time) / (self.reqs_count as f64)
                    )))
                    .push(Text::new(format!(
                        "Avg memory: {} Kb",
                        (self.memory) / (self.reqs_count as f64)
                    ))),
            )
            .padding(20)
            .max_width(400)
            .into()
    }
}
