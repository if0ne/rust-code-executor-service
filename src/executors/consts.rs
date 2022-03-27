#[cfg(windows)]
pub const CONSOLE_CALL: &str = "cmd";
#[cfg(not(windows))]
pub const CONSOLE_CALL: &str = "sh";

#[cfg(windows)]
pub const CONSOLE_ARG: &str = "/C";
#[cfg(not(windows))]
pub const CONSOLE_ARG: &str = "-c";

pub const SOURCE_FILE_NAME: &str = "code";

#[cfg(windows)]
pub const COMPILED_FILE_NAME: &str = "compiled_file.exe";
#[cfg(not(windows))]
pub const COMPILED_FILE_NAME: &str = "compiled_file";

#[cfg(windows)]
pub const OS_PATH_PREFIX: &str = "";
#[cfg(not(windows))]
pub const OS_PATH_PREFIX: &str = "/usr/src/app/";
