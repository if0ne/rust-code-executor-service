/// Команда для вызова консоли
#[cfg(windows)]
pub const CONSOLE_CALL: &str = "cmd";
#[cfg(not(windows))]
pub const CONSOLE_CALL: &str = "sh";

/// Команда для закрытия консоли после завершения выполнения программы в ней
#[cfg(windows)]
pub const CONSOLE_ARG: &str = "/C";
#[cfg(not(windows))]
pub const CONSOLE_ARG: &str = "-c";

/// Стандартное названия файла с исходным кодом пользователя
pub const SOURCE_FILE_NAME: &str = "code";

/// Стандартное название скомпилированного файла
#[cfg(windows)]
pub const COMPILED_FILE_NAME: &str = "compiled_file.exe";
#[cfg(not(windows))]
pub const COMPILED_FILE_NAME: &str = "compiled_file";

/// Префикс для пути к скомпилированному файлу
#[cfg(windows)]
pub const OS_PATH_PREFIX: &str = "";
#[cfg(not(windows))]
pub const OS_PATH_PREFIX: &str = "/usr/src/app/";
