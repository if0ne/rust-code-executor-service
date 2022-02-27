use crate::mesure::{ProcessInfo, ProcessInformer};

use std::error::Error;
use std::mem;

use std::os::windows::io::AsRawHandle;
use std::time::Duration;

impl ProcessInformer for std::process::Child {
    fn get_process_info(&mut self) -> Result<ProcessInfo, Box<dyn Error>> {
        let instance = self.as_raw_handle() as isize;
        let _status = self.wait()?;

        let (user_time, kernel_time) = unsafe {
            let mut ctime = mem::zeroed();
            let mut etime = mem::zeroed();
            let mut kernel_time = mem::zeroed();
            let mut user_time = mem::zeroed();
            let res = windows::Win32::System::Threading::GetProcessTimes(
                windows::Win32::Foundation::HANDLE(instance),
                &mut ctime,
                &mut etime,
                &mut kernel_time,
                &mut user_time,
            );

            if res.as_bool() {
                //Умножение на 100 для перевода в наносекунды, т.к. функция возвращает время измеряемой в 100 наносекундах
                let user = (((user_time.dwHighDateTime as i64) << 32)
                    + user_time.dwLowDateTime as i64)
                    * 100;
                let kernel = (((kernel_time.dwHighDateTime as i64) << 32)
                    + kernel_time.dwLowDateTime as i64)
                    * 100;
                (user as u64, kernel as u64)
            } else {
                (0, 0)
            }
        };

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
            user_time: Duration::from_nanos(user_time),
            kernel_time: Duration::from_nanos(kernel_time),
            total_memory: total_bytes,
        })
    }
}
