use thiserror::Error;

#[derive(Error, Debug)]
pub enum ProcessError {
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    #[error("Processing failed: {0}")]
    ProcessingFailed(String),
}

/// Process input data and return result
/// 
/// # Example
/// ```
/// use my_cli::processor::process_data;
/// let result = process_data("hello").unwrap();
/// assert!(!result.is_empty());
/// ```
pub fn process_data(input: &str) -> Result<String, ProcessError> {
    if input.is_empty() {
        return Err(ProcessError::InvalidInput("Input cannot be empty".to_string()));
    }

    // TODO: Implement your processing logic here
    let result = format!("Processed: {}", input.to_uppercase());
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_data() {
        let result = process_data("test").unwrap();
        assert_eq!(result, "Processed: TEST");
    }

    #[test]
    fn test_empty_input() {
        let result = process_data("");
        assert!(result.is_err());
    }
}
