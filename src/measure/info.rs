use crate::measure::{ProcessInfo, ProcessInformer};

use std::error::Error;
use std::io::{BufRead, BufReader};
use wait4;
use wait4::Wait4;

impl ProcessInformer for std::process::Child {
    fn get_process_info(mut self) -> Result<ProcessInfo, Box<dyn Error>> {
        let output = BufReader::new(self.stdout.take().unwrap());
        let work_result = self.wait4().unwrap();
        let duration = work_result.rusage.utime + work_result.rusage.stime;
        let exit_status = work_result.status.code().unwrap();
        let total_bytes = work_result.rusage.maxrss;

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
