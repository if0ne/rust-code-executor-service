use std::time::Duration;

#[derive(Debug)]
pub struct ProcessInfo {
    pub user_time: Duration,
    pub kernel_time: Duration,
    pub total_memory: u64,
}

pub trait ProcessInformer {
    fn get_process_info(&mut self) -> Result<ProcessInfo, Box<dyn std::error::Error>>;
}

#[cfg(windows)]
pub mod win_info;

#[cfg(not(windows))]
pub mod unix_info;
