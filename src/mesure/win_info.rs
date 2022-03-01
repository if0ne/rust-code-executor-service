use crate::mesure::{ProcessInfo, ProcessInformer};

use std::error::Error;
use std::mem;

use std::os::windows::io::AsRawHandle;

#[async_trait::async_trait]
impl ProcessInformer for std::process::Child {
    async fn get_process_info(&mut self) -> Result<ProcessInfo, Box<dyn Error>> {
        let instance = self.as_raw_handle() as isize;
        let start = std::time::Instant::now();
        let _status = self.wait()?;
        let delta = start.elapsed();

        let total_bytes = unsafe {
            let mut pmc = mem::zeroed();
            let res = windows::Win32::System::ProcessStatus::K32GetProcessMemoryInfo(
                windows::Win32::Foundation::HANDLE(instance),
                &mut pmc,
                std::mem::size_of::<windows::Win32::System::ProcessStatus::PROCESS_MEMORY_COUNTERS>(
                ) as u32,
            );
            if res.as_bool() {
                pmc.PeakWorkingSetSize as u64
            } else {
                0
            }
        };

        Ok(ProcessInfo {
            execute_time: delta,
            total_memory: total_bytes,
        })
    }
}

#[async_trait::async_trait]
impl ProcessInformer for tokio::process::Child {
    async fn get_process_info(&mut self) -> Result<ProcessInfo, Box<dyn Error>> {
        let instance = self.raw_handle().unwrap() as isize;
        let start = std::time::Instant::now();
        let status = self.wait();

        let total_bytes = unsafe {
            let mut pmc = mem::zeroed();
            let res = windows::Win32::System::ProcessStatus::K32GetProcessMemoryInfo(
                windows::Win32::Foundation::HANDLE(instance),
                &mut pmc,
                std::mem::size_of::<windows::Win32::System::ProcessStatus::PROCESS_MEMORY_COUNTERS>(
                ) as u32,
            );
            if res.as_bool() {
                pmc.PeakWorkingSetSize as u64
            } else {
                0
            }
        };

        status.await?;
        let delta = start.elapsed();

        Ok(ProcessInfo {
            execute_time: delta,
            total_memory: total_bytes,
        })
    }
}
