use std::collections::HashMap;

pub struct RequestInfo {
    pub langs: HashMap<String, bool>,
    pub count: u32,
    pub test_name: String,
    pub delay: u32,
    pub timeout: u32,
}

impl RequestInfo {
    pub fn new(
        langs: HashMap<String, bool>,
        count: u32,
        test_name: String,
        delay: u32,
        timeout: u32,
    ) -> Self {
        RequestInfo {
            langs,
            count,
            test_name,
            delay,
            timeout,
        }
    }
}
