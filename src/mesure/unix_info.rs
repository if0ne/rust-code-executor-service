use crate::mesure::{ProcessInfo, ProcessInformer};

use std::error::Error;
use std::time::Duration;

fn timeval_to_duration(val: libc::timeval) -> Duration {
    let v = i64::from(val.tv_sec) * 1_000_000 + i64::from(val.tv_usec);
    Duration::from_micros(v as u64)
}

#[async_trait::async_trait]
impl ProcessInformer for std::process::Child {
    async fn get_process_info(&mut self) -> Result<ProcessInfo, Box<dyn Error>> {
        drop(self.stdin.take());
        let pid = self.id() as i32;
        let mut status = 0;
        let options = 0;
        let mut info = std::mem::MaybeUninit::zeroed();

        let res = unsafe { libc::wait4(pid, &mut status, options, info.as_mut_ptr()) };

        if res < 0 {
            Err(Box::new(std::io::Error::last_os_error()))
        } else {
            let info = unsafe { info.assume_init() };

            Ok(ProcessInfo {
                execute_time: timeval_to_duration(info.ru_utime),
                total_memory: info.ru_maxrss as u64,
            })
        }
    }
}
