use std::fs;


pub struct FileWriter {
    path : String
}

impl FileWriter {
    pub(crate) fn new(path : &str) -> FileWriter {
        let p = path.to_string();
        FileWriter { path : p }
    }
    
    pub fn write(&self, content : &str) -> Result<(), String> {
        match fs::write(&self.path, content) {
            Ok(_) => Ok(()),
            Err(e) => Err(e.to_string())
        }
    }
}