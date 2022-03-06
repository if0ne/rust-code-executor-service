#[macro_export]
macro_rules! make_compiler {
    ($executor:ty) => {
        use crate::executors::Uncompiled;
        use crate::executors::{DefinedLanguage, Executor};
        use core::marker::PhantomData;
        impl From<$executor> for DefinedLanguage {
            fn from(exec: $executor) -> Self {
                DefinedLanguage::Compiled(Executor {
                    inner: Box::new(exec),
                    state: PhantomData::<Uncompiled>,
                })
            }
        }
    };
}

#[macro_export]
macro_rules! make_interpreter {
    ($executor:ty) => {
        use crate::executors::Interpreted;
        use crate::executors::{DefinedLanguage, Executor};
        use core::marker::PhantomData;
        impl From<$executor> for DefinedLanguage {
            fn from(exec: $executor) -> Self {
                DefinedLanguage::Interpreted(Executor {
                    inner: Box::new(exec),
                    state: PhantomData::<Interpreted>,
                })
            }
        }
    };
}
