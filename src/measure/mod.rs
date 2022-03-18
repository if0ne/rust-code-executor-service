use std::time::Duration;

#[derive(Debug)]
pub struct ProcessInfo {
    pub execute_time: Duration,
    pub total_memory: u64,
    pub output: String,
    pub exit_status: i32,
}

pub trait ProcessInformer {
    fn get_process_info(self) -> Result<ProcessInfo, Box<dyn std::error::Error>>;
}

pub mod info;
