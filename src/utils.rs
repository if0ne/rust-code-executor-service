use crate::routes::execute_service::executed_test::ExecuteStatus;

pub fn read_from_buffer<T: std::io::Read>(
    mut buffer: std::io::BufReader<T>,
) -> Result<String, crate::routes::execute_service::executed_test::ExecuteStatus> {
    use std::io::Read;

    let mut out = Vec::new();
    buffer
        .read_to_end(&mut out)
        .map_err(|_| ExecuteStatus::IoFail)?;
    let result = String::from_utf8_lossy(&out).to_string();

    Ok(result)
}
