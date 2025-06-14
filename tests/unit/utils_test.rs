#[cfg(test)]
mod utils_tests {
    use std::env;
    use std::path::PathBuf;
    use muv::utils;
    use anyhow::Result;
    
    #[test]
    fn test_get_muv_home_with_env_var() -> Result<()> {
        // Set a custom MUV_HOME for this test
        let temp_path = "/tmp/muv_test_home";
        env::set_var("MUV_HOME", temp_path);
        
        let home = utils::get_muv_home()?;
        assert_eq!(home, PathBuf::from(temp_path));
        
        // Clean up
        env::remove_var("MUV_HOME");
        Ok(())
    }
    
    #[test]
    fn test_get_muv_home_default() -> Result<()> {
        // Ensure MUV_HOME is not set
        env::remove_var("MUV_HOME");
        
        let home = utils::get_muv_home()?;
        let expected_base = dirs::data_dir().or_else(dirs::home_dir).unwrap();
        let expected = expected_base.join(".muv");
        
        assert_eq!(home, expected);
        Ok(())
    }
    
    #[test]
    fn test_check_uv_exists() {
        // This test assumes uv is installed on the test system
        let result = utils::check_uv_exists();
        assert!(result.is_ok());
    }
}