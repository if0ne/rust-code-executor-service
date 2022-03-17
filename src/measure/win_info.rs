use crate::measure::{ProcessInfo, ProcessInformer};

use std::error::Error;
use std::io::{BufRead, BufReader};
use std::mem;
use wait4;

use std::os::windows::io::AsRawHandle;
use wait4::Wait4;

fn get_windows_process_memory(instance: isize) -> u64 {
    unsafe {
        let mut pmc = mem::zeroed();
        let res = windows::Win32::System::ProcessStatus::K32GetProcessMemoryInfo(
            windows::Win32::Foundation::HANDLE(instance),
            &mut pmc,
            std::mem::size_of::<windows::Win32::System::ProcessStatus::PROCESS_MEMORY_COUNTERS>()
                as u32,
        );
        if res.as_bool() {
            pmc.PeakWorkingSetSize as u64
        } else {
            0
        }
    }
}

#[async_trait::async_trait]
impl ProcessInformer for std::process::Child {
    async fn get_process_info(mut self) -> Result<ProcessInfo, Box<dyn Error>> {
        let start = std::time::Instant::now();
        let output = BufReader::new(self.stdout.take().unwrap());
        let work_result = self.wait4().unwrap();
        let duration = start.elapsed();
        let exit_status = work_result.status.code().unwrap();
        let total_bytes = work_result.rusage.maxrss; //get_windows_process_memory(instance);

        let readed = output
            .lines()
            .map(|line| line.unwrap())
            .collect::<Vec<_>>()
            .join("\n");

        Ok(ProcessInfo {
            execute_time: duration,
            total_memory: total_bytes,
            output: readed,
            exit_status,
        })
    }
}

#[async_trait::async_trait]
impl ProcessInformer for tokio::process::Child {
    async fn get_process_info(mut self) -> Result<ProcessInfo, Box<dyn Error>> {
        let instance = self.raw_handle().unwrap() as isize;

        let start = std::time::Instant::now();
        let status = self.wait_with_output();

        let total_bytes = get_windows_process_memory(instance);

        let output = status.await?;
        let delta = start.elapsed();

        Ok(ProcessInfo {
            execute_time: delta,
            total_memory: total_bytes,
            output: String::from_utf8_lossy(&output.stdout).to_string(),
            exit_status: output.status.code().unwrap_or(-1),
        })
    }
}
