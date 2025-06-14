#[cfg(test)]
mod error_tests {
    use muv::error::MuvError;
    use std::path::PathBuf;
    
    #[test]
    fn test_error_display() {
        let error = MuvError::EnvironmentNotFound("test_env".to_string());
        assert!(format!("{}", error).contains("test_env"));
        
        let error = MuvError::HomeDirError;
        assert!(!format!("{}", error).is_empty());
        
        let error = MuvError::IoError(std::io::Error::new(std::io::ErrorKind::NotFound, "test error"));
        assert!(format!("{}", error).contains("IO error"));
    }
    
    #[test]
    fn test_error_conversion() {
        let io_error = std::io::Error::new(std::io::ErrorKind::NotFound, "test error");
        let muv_error: MuvError = MuvError::IoError(io_error);
        assert!(format!("{}", muv_error).contains("IO error"));
    }
}