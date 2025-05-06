use std::fs;

trait FileSystemManager {
    fn list_files(dir: &str) -> Result<Vec<String>, String>;
    fn read_file(path: &str) -> Result<String, String>;
    fn copy_file(src: &str, dst: &str) -> Result<(), String>;
    fn delete_file(path: &str) -> Result<(), String>;
    fn exists(path: &str) -> Result<bool, String>;
}

impl FileSystemManager for FileSystemManager {
    fn list_files(dir: &str) -> Result<Vec<String>, String> {
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

    fn read_file(path: &str) -> Result<String, String> {
        match fs::read(path) {
            Ok(content) => Ok(String::from_utf8(content).unwrap()),
            Err(error) => Err(error.to_string())
        }
    }

    fn copy_file(src: &str, dst: &str) -> Result<(), String> {
        match fs::copy(src, dst) {
            Ok(_) => Ok(),
            Err(error) => Err(error.to_string())
        }
    }

    fn delete_file(path: &str) -> Result<(), String> {
        match fs::remove_file(path) {
            Ok(_) => Ok(),
            Err(error) => Err(error.to_string())
        }
    }

    fn exists(path: &str) -> Result<bool, String> {
        match fs::exists(path) {
            Ok(exists) => Ok(exists),
            Err(error) => Err(error.to_string())
        }
    }
}
