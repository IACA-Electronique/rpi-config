use std::fs;

pub trait FileSystemManager {
    fn list_files(&self,dir: &str) -> Result<Vec<String>, String>;
    fn read_file(&self,path: &str) -> Result<String, String>;
    fn copy_file(&self,src: &str, dst: &str) -> Result<(), String>;
    fn delete_file(&self,path: &str) -> Result<(), String>;
    fn exists(&self, path: &str) -> Result<bool, String>;
    fn create_dir_if_not_exists(&self, path: &str) -> Result<(), String>;
}

pub struct DefaultFileSystemManager;

impl FileSystemManager for DefaultFileSystemManager {
    fn list_files(&self,dir: &str) -> Result<Vec<String>, String> {
        match fs::read_dir(dir) {
            Ok(entries) => {
                let mut files = Vec::new();
                for entry in entries {
                    let file_name = entry.unwrap().file_name().into_string().unwrap();
                    files.push(file_name);
                }
                Ok(files)
            }
            Err(e) => Err(e.to_string())
        }
    }

    fn read_file(&self,path: &str) -> Result<String, String> {
        match fs::read(path) {
            Ok(content) => Ok(String::from_utf8(content).unwrap()),
            Err(error) => Err(error.to_string())
        }
    }

    fn copy_file(&self,src: &str, dst: &str) -> Result<(), String> {
        match fs::copy(src, dst) {
            Ok(_) => Ok(()),
            Err(error) => Err(error.to_string())
        }
    }

    fn delete_file(&self,path: &str) -> Result<(), String> {
        match fs::remove_file(path) {
            Ok(_) => Ok(()),
            Err(error) => Err(error.to_string())
        }
    }

    fn exists(&self, path: &str) -> Result<bool, String> {
        match fs::exists(path) {
            Ok(exists) => Ok(exists),
            Err(error) => Err(error.to_string())
        }
    }

    fn create_dir_if_not_exists(&self, path: &str) -> Result<(), String> {
        match fs::create_dir_all(path) {
            Ok(_) => Ok(()),
            Err(error) => Err(error.to_string())
        }   
    }
}
