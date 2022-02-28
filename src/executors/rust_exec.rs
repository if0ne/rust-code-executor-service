use crate::executors::{Defined, Executor, ExecutorImpl};
use crate::routes::compile::{ExecuteStats, ExecutedTest, Solution};
use crate::ProcessInformer;
use std::io::Write;
use std::marker::PhantomData;
use std::path::Path;

pub struct RustExecutor {
    pub(crate) path: String,
}

unsafe impl Sync for RustExecutor {}
unsafe impl Send for RustExecutor {}

#[async_trait::async_trait]
impl ExecutorImpl for RustExecutor {
    async fn compile(&mut self, solution: &Solution) -> Result<(), ()> {
        let folder = format!("{}_{}", solution.get_uuid(), solution.get_hash());
        if Path::new(&folder).exists() {
            return Err(());
        }

        {
            std::fs::create_dir(&folder).map_err(|_| ())?;
            let mut solution_file =
                std::fs::File::create(format!("{}/{}", folder, "code.txt")).map_err(|_| ())?;
            solution_file
                .write_all(solution.get_src().as_bytes())
                .unwrap();
        }

        let _ = std::process::Command::new("rustc")
            .arg("-O")
            .arg(format!("{}/{}", folder, "code.txt"))
            .arg("--out-dir")
            .arg(format!("{}", folder))
            .spawn()
            .unwrap()
            .wait()
            .unwrap();

        self.path = folder;
        Ok(())
    }

    async fn execute(&self, test: &str) -> ExecutedTest {
        let mut process = std::process::Command::new("cmd")
            .current_dir(&self.path)
            .arg("/C")
            .arg( "code.exe")
            .current_dir(&self.path)
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .spawn()
            .unwrap();

        process
            .stdin
            .as_mut()
            .unwrap()
            .write_all(test.as_ref())
            .unwrap();
        let program_info = process.get_process_info().await.unwrap();
        let output = process.wait_with_output().unwrap();

        ExecutedTest {
            time: program_info.execute_time.as_millis() as u64,
            memory: program_info.total_memory / 1024,
            result: String::from_utf8_lossy(&output.stdout).to_string(),
            status: ExecuteStats::OK,
        }
    }

    async fn clean(&self) {
        std::fs::remove_dir_all(&self.path).unwrap();
    }
}

impl From<RustExecutor> for Executor<Defined> {
    fn from(exec: RustExecutor) -> Self {
        Executor {
            inner: Box::new(exec),
            state: PhantomData::<Defined>,
        }
    }
}
