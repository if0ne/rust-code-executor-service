pub fn read_from_buffer<T: std::io::Read>(
    buffer: std::io::BufReader<T>,
) -> Result<String, crate::routes::execute_service::executed_test::ExecuteStatus> {
    use crate::routes::execute_service::executed_test::ExecuteStatus;
    use std::io::BufRead;

    let buffer = buffer.lines().collect::<Vec<_>>();

    for line in buffer.iter() {
        if line.is_err() {
            return Err(ExecuteStatus::IoFail);
        }
    }

    let buffer = buffer
        .into_iter()
        .map(|line| line.unwrap())
        .collect::<Vec<_>>()
        .join("\n");

    Ok(buffer)
}