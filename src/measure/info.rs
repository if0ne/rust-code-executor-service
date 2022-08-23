use crate::measure::{ProcessInfo, ProcessInformer};
use crate::models::executed_test::ExecuteStatus;
use crate::utils::read_from_buffer;
use std::io::BufReader;
use wait4::Wait4;

#[allow(clippy::single_match)]
impl ProcessInformer for std::process::Child {
    fn get_process_info(
        mut self,
        timeout: std::time::Duration,
    ) -> Result<ProcessInfo, ExecuteStatus> {
        // Буфер стандартного потока вывода
        let stdout = BufReader::new(self.stdout.take().ok_or(ExecuteStatus::IoFail)?);
        let stderr = BufReader::new(self.stderr.take().ok_or(ExecuteStatus::IoFail)?);

        // Получение дескриптора потока (для unix - id)
        let pid = get_pid(&self);

        // Запуск процесса в отдельном потоке для решения зацикливания процесса
        let (sender, receiver) = std::sync::mpsc::channel();
        let process = std::thread::spawn(move || {
            let work_result = self.wait4().map_err(|_| ExecuteStatus::RuntimeError);
            match sender.send(work_result) {
                Ok(_) => {}  // Значение отправилось
                Err(_) => {} // Процесс зациклился, либо выполняется дольше положенного времени
            }
        });

        //TODO: Асинхронный вызов
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

        let stdout = read_from_buffer(stdout)?;
        let stderr = read_from_buffer(stderr)?;

        Ok(ProcessInfo {
            execute_time: duration,
            total_memory: total_bytes,
            stdout,
            exit_status,
            stderr,
        })
    }
}

/// Получение "дескриптора" в Windows
#[cfg(windows)]
fn get_pid(child: &std::process::Child) -> windows::Win32::Foundation::HANDLE {
    use std::os::windows::io::AsRawHandle;
    windows::Win32::Foundation::HANDLE(child.as_raw_handle() as isize)
}

/// Получение "дескриптора" в Unix
#[cfg(not(windows))]
fn get_pid(child: &std::process::Child) -> i32 {
    child.id() as i32
}

/// Принудительное завершение процесса в Windows
#[cfg(windows)]
fn kill_process(thread: std::thread::JoinHandle<()>, handle: windows::Win32::Foundation::HANDLE) {
    while !thread.is_finished() {
        unsafe {
            windows::Win32::System::Threading::TerminateProcess(handle, 1);
        }
    }
}

/// Принудительное завершение процесса в Unix
#[cfg(not(windows))]
fn kill_process(thread: std::thread::JoinHandle<()>, pid: i32) {
    while !thread.is_finished() {
        unsafe {
            libc::kill(pid, libc::SIGKILL);
        }
    }
}
