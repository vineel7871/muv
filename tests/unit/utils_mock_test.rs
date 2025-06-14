#[cfg(test)]
mod utils_mock_tests {
    use muv::utils;
    use std::env;
    use std::path::PathBuf;
    use tempfile::TempDir;
    
    fn setup() -> TempDir {
        let temp_dir = tempfile::tempdir().expect("Failed to create temp directory");
        std::env::set_var("MUV_HOME", temp_dir.path().to_str().unwrap());
        temp_dir
    }
    
    fn cleanup() {
        std::env::remove_var("MUV_HOME");
    }
    
    #[test]
    fn test_get_envs_dir_creates_directory() {
        let temp_dir = setup();
        
        // Get envs dir should create the directory if it doesn't exist
        let envs_dir = utils::get_envs_dir().expect("Failed to get envs dir");
        assert!(envs_dir.exists());
        assert!(envs_dir.is_dir());
        assert_eq!(envs_dir, PathBuf::from(temp_dir.path()).join(".muv").join("envs"));
        
        cleanup();
    }
    
    #[test]
    fn test_get_env_path() {
        let temp_dir = setup();
        
        let env_path = utils::get_env_path("test_env").expect("Failed to get env path");
        assert_eq!(
            env_path, 
            PathBuf::from(temp_dir.path()).join(".muv").join("envs").join("test_env")
        );
        
        cleanup();
    }
    
    #[test]
    fn test_ensure_env_exists_fails_for_nonexistent() {
        let temp_dir = setup();
        
        let result = utils::ensure_env_exists("nonexistent_env");
        assert!(result.is_err());
        
        cleanup();
    }
}