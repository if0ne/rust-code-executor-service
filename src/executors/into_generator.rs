#[macro_export]
macro_rules! make_compiler {
    ($executor:ty) => {
        use crate::executors::defined_language::DefinedLanguage;
        use crate::executors::executor::Executor;
        impl From<$executor> for DefinedLanguage {
            fn from(exec: $executor) -> Self {
                DefinedLanguage::Compiled(Executor::new(Box::new(exec)))
            }
        }
    };
}

#[macro_export]
macro_rules! make_interpreter {
    ($executor:ty) => {
        use crate::executors::defined_language::DefinedLanguage;
        use crate::executors::executor::Executor;
        impl From<$executor> for DefinedLanguage {
            fn from(exec: $executor) -> Self {
                DefinedLanguage::Interpreted(Executor::new(Box::new(exec)))
            }
        }
    };
}
