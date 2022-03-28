use crate::measure::{ProcessInfo, ProcessInformer};

use crate::routes::execute_service::executed_test::ExecuteStatus;
use std::io::{BufRead, BufReader};
use wait4::Wait4;

impl ProcessInformer for std::process::Child {
    fn get_process_info(mut self) -> Result<ProcessInfo, ExecuteStatus> {
        let output = BufReader::new(self.stdout.take().ok_or(ExecuteStatus::IoFail)?);
        let work_result = self.wait4().map_err(|_| ExecuteStatus::RuntimeError)?;
        let duration = work_result.rusage.utime + work_result.rusage.stime;
        let exit_status = work_result.status.code().unwrap_or(-1);
        let total_bytes = work_result.rusage.maxrss;

        let read = output.lines().collect::<Vec<_>>();

        for line in read.iter() {
            if line.is_err() {
                return Err(ExecuteStatus::IoFail);
            }
        }

        let read = read
            .into_iter()
            .map(|line| line.unwrap())
            .collect::<Vec<_>>()
            .join("\n");

        Ok(ProcessInfo {
            execute_time: duration,
            total_memory: total_bytes,
            output: read,
            exit_status,
        })
    }
}
