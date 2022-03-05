use std::time::Duration;

#[derive(Debug)]
pub struct ProcessInfo {
    pub execute_time: Duration,
    pub total_memory: u64,
    pub output: String,
    pub exit_status: i32,
}

#[async_trait::async_trait]
pub trait ProcessInformer {
    async fn get_process_info(mut self) -> Result<ProcessInfo, Box<dyn std::error::Error>>;
}

#[cfg(windows)]
pub mod win_info;

#[cfg(not(windows))]
pub mod unix_info;
