use std::cell::RefCell;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

// Mock for environment variables
pub struct MockEnvVars {
    vars: RefCell<HashMap<String, String>>,
}

impl MockEnvVars {
    pub fn new() -> Self {
        Self {
            vars: RefCell::new(HashMap::new()),
        }
    }

    pub fn set(&self, key: &str, value: &str) {
        self.vars.borrow_mut().insert(key.to_string(), value.to_string());
    }

    pub fn get(&self, key: &str) -> Option<String> {
        self.vars.borrow().get(key).cloned()
    }

    pub fn remove(&self, key: &str) {
        self.vars.borrow_mut().remove(key);
    }
}

// Mock for file system operations
pub struct MockFileSystem {
    files: RefCell<HashMap<PathBuf, Vec<u8>>>,
    directories: RefCell<Vec<PathBuf>>,
}

impl MockFileSystem {
    pub fn new() -> Self {
        Self {
            files: RefCell::new(HashMap::new()),
            directories: RefCell::new(Vec::new()),
        }
    }

    pub fn create_file(&self, path: &Path, content: &[u8]) {
        self.files.borrow_mut().insert(path.to_path_buf(), content.to_vec());
    }

    pub fn create_dir(&self, path: &Path) {
        self.directories.borrow_mut().push(path.to_path_buf());
    }

    pub fn file_exists(&self, path: &Path) -> bool {
        self.files.borrow().contains_key(&path.to_path_buf())
    }

    pub fn dir_exists(&self, path: &Path) -> bool {
        self.directories.borrow().contains(&path.to_path_buf())
    }

    pub fn read_file(&self, path: &Path) -> Option<Vec<u8>> {
        self.files.borrow().get(&path.to_path_buf()).cloned()
    }
}