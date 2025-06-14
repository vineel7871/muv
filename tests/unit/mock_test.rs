#[cfg(test)]
mod mock_tests {
    use crate::mock::{MockEnvVars, MockFileSystem};
    use std::path::Path;
    
    #[test]
    fn test_mock_env_vars() {
        let mock_env = MockEnvVars::new();
        
        // Test setting and getting variables
        mock_env.set("TEST_VAR", "test_value");
        assert_eq!(mock_env.get("TEST_VAR"), Some("test_value".to_string()));
        
        // Test removing variables
        mock_env.remove("TEST_VAR");
        assert_eq!(mock_env.get("TEST_VAR"), None);
    }
    
    #[test]
    fn test_mock_file_system() {
        let mock_fs = MockFileSystem::new();
        let test_path = Path::new("/test/file.txt");
        let test_dir = Path::new("/test");
        
        // Test creating and checking files
        mock_fs.create_file(test_path, b"test content");
        assert!(mock_fs.file_exists(test_path));
        assert_eq!(mock_fs.read_file(test_path), Some(b"test content".to_vec()));
        
        // Test creating and checking directories
        mock_fs.create_dir(test_dir);
        assert!(mock_fs.dir_exists(test_dir));
    }
}