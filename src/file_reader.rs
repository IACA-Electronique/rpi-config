use std::fs;

pub struct FileReader {
    path: String,
}

impl FileReader {
    pub fn new(path: &str) -> FileReader {
        FileReader {
            path: path.to_string(),
        }
    }

    pub fn read(&self) -> Result<String, String> {
        return match fs::read(&self.path) {
            Ok(value) => Ok(String::from_utf8(value).unwrap()),
            Err(_) => Err("Error reading file".to_string()),
        };
    }
}
