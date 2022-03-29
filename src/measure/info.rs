use crate::measure::{ProcessInfo, ProcessInformer};
use crate::routes::execute_service::executed_test::ExecuteStatus;
use std::io::{BufRead, BufReader};
use std::os::windows::io::AsRawHandle;
use wait4::Wait4;

impl ProcessInformer for std::process::Child {
    fn get_process_info(
        mut self,
        timeout: std::time::Duration,
    ) -> Result<ProcessInfo, ExecuteStatus> {
        let output = BufReader::new(self.stdout.take().ok_or(ExecuteStatus::IoFail)?);

        let pid = get_pid(&self);

        let (sender, receiver) = std::sync::mpsc::channel();
        let process = std::thread::spawn(move || {
            let work_result = self.wait4().map_err(|_| ExecuteStatus::RuntimeError);
            match sender.send(work_result) {
                Ok(_) => {}
                Err(_) => {}
            }
        });

        let work_result = receiver
            .recv_timeout(timeout)
            .map_err(|_| ExecuteStatus::Timeout);

        if work_result.is_err() {
            kill_process(process, pid);
        }

        let work_result = work_result??;
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

#[cfg(windows)]
fn get_pid(child: &std::process::Child) -> windows::Win32::Foundation::HANDLE {
    windows::Win32::Foundation::HANDLE(child.as_raw_handle() as isize)
}

#[cfg(not(windows))]
fn get_pid(child: &std::process::Child) -> i32 {
    self.id() as i32
}

#[cfg(windows)]
fn kill_process(thread: std::thread::JoinHandle<()>, handle: windows::Win32::Foundation::HANDLE) {
    while !thread.is_finished() {
        unsafe {
            windows::Win32::System::Threading::TerminateProcess(handle, 1);
        }
    }
}

#[cfg(not(windows))]
fn kill_process(thread: std::thread::JoinHandle<()>, pid: i32) {
    while !thread.is_finished() {
        unsafe {
            libc::kill(pid, libc::SIGKILL);
        }
    }
}
