use crate::routes::compile::{ExecuteStats, ExecutedTest, Solution};
use crate::ProcessInformer;
use std::io::Write;
use std::marker::PhantomData;
use std::path::Path;

pub mod rust_exec;

pub const COMPILE_FILE_NAME: &str = "code";

pub struct Defined;
pub struct Compiled;

#[async_trait::async_trait]
trait ExecutorImpl: Send + Sync {
    fn get_compiler_args(&self, solution: &Solution) -> Vec<String>;
    fn get_execute_args(&self) -> Vec<String>;

    async fn compile(&mut self, solution: &Solution) -> Result<(), ()> {
        let folder = solution.get_folder_name();
        if Path::new(&folder).exists() {
            return Err(());
        }

        {
            std::fs::create_dir(&folder).unwrap();
            let mut solution_file =
                std::fs::File::create(format!("{}/{}", folder, COMPILE_FILE_NAME)).unwrap();
            solution_file
                .write_all(solution.get_src().as_bytes())
                .unwrap();
        }

        let compiler_args = self.get_compiler_args(solution);

        let _ = std::process::Command::new("cmd")
            .arg("/C")
            .args(compiler_args)
            .spawn()
            .unwrap()
            .wait()
            .map_err(|_| ())?;

        Ok(())
    }

    async fn execute(&self, solution: &Solution, test: &str) -> ExecutedTest {
        let folder = solution.get_folder_name();
        let execute_args = self.get_execute_args();

        let mut process = std::process::Command::new("cmd")
            .current_dir(&folder)
            .arg("/C")
            .args(execute_args)
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
            time: program_info.execute_time.as_millis(),
            memory: program_info.total_memory / 1024,
            result: String::from_utf8_lossy(&output.stdout).to_string(),
            status: ExecuteStats::OK,
        }
    }

    async fn clean(&self, solution: &Solution) {
        let folder = solution.get_folder_name();
        std::fs::remove_dir_all(&folder).unwrap();
    }
}

unsafe impl Send for Executor<Defined> {}
unsafe impl Sync for Executor<Defined> {}

unsafe impl Send for Executor<Compiled> {}
unsafe impl Sync for Executor<Compiled> {}

impl Executor<Defined> {
    pub async fn compile(mut self, solution: &Solution) -> Result<Executor<Compiled>, ()> {
        self.inner.compile(solution).await?;

        Ok(Executor {
            inner: self.inner,
            state: PhantomData::<Compiled>,
        })
    }
}

impl Executor<Compiled> {
    pub async fn execute(&self, solution: &Solution, test: &str) -> ExecutedTest {
        self.inner.execute(solution, test).await
    }
    pub async fn clean(self, solution: &Solution) {
        self.inner.clean(solution).await;
    }
}

pub struct Executor<S> {
    inner: Box<dyn ExecutorImpl>,
    state: std::marker::PhantomData<S>,
}
