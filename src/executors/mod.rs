use std::marker::PhantomData;
use crate::routes::compile::{ExecutedTest, Solution};

pub mod rust_exec;

pub struct Defined;
pub struct Compiled;

#[async_trait::async_trait]
trait ExecutorImpl: Send + Sync {
    async fn compile(&mut self, solution: &Solution) -> Result<(), ()>;
    async fn execute(&self, test: &str) -> ExecutedTest;
    async fn clean(&self);
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
            state: PhantomData::<Compiled>
        })
    }
}

impl Executor<Compiled> {
    pub async fn execute(&self, test: &str) -> ExecutedTest {
        self.inner.execute(test).await
    }
    pub async fn clean(self) {
        self.inner.clean().await;
    }
}

pub struct Executor<S> {
    inner: Box<dyn ExecutorImpl>,
    state: std::marker::PhantomData<S>,
}
