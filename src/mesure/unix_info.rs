use crate::mesure::{ProcessInfo, ProcessInformer};
use std::time::Duration;

fn timeval_to_duration(val: libc::timeval) -> Duration {
    let v = i64::from(val.tv_sec) * 1_000_000 + i64::from(val.tv_usec);
    Duration::from_micros(v as u64)
}

impl ProcessInformer for std::process::Child {
    fn get_process_info(&mut self) -> Result<ProcessInfo, Box<dyn Error>> {
        drop(self.stdin.take());
        let pid = self.id() as i32;
        let mut status = 0;
        let options = 0;
        let mut info = std::mem::MaybeUninit::zeroed();

        let res = unsafe { libc::wait4(pid, &mut status, options, info) };

        if res < 0 {
            Err(std::io::Error::last_os_error())
        } else {
            let info = unsafe { info.assume_init() };

            Ok(ProcessInfo {
                user_time: timeval_to_duration(info.ru_utime),
                kernel_time: timeval_to_duration(info.ru_stime),
                total_memory: info.ru_maxrss as u64,
            })
        }
    }
}
