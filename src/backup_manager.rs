use crate::low_level::filesystem_manager::FileSystemManager;
use chrono::Local;
use rand::Rng;

pub struct BackupManager {
    filesystem: Box<dyn FileSystemManager>,
    path: String,
    directory: String,
}

impl BackupManager {
    pub fn new(
        path: &str,
        directory: &str,
        filesystem_manager: Box<dyn FileSystemManager>,
    ) -> Self {
        Self {
            path: path.to_string(),
            directory: directory.to_string(),
            filesystem: filesystem_manager,
        }
    }

    pub fn create(&self) -> Result<(), String> {
        let random_number = rand::rng().random_range(0..=99);
        let timestamp = Local::now().format("%Y%m%d_%H%M%S").to_string();
        let backup_path = format!(
            "{}/backup_{}_{}.bak",
            self.directory, timestamp, random_number
        );
        
        if self.filesystem.create_dir_if_not_exists(&self.directory).is_ok() {
            match self.filesystem.copy_file(&self.path, &backup_path) {
                Ok(_) => Ok(()),
                Err(e) => Err(format!("Failed to create backup: {}", e)),
            }   
        }else { 
            Err("Unable to create backup directory".to_string()) 
        }
    }
    
    pub fn list(&self) -> Result<Vec<String>, String> {
        let mut backups = Vec::new();
        match self.filesystem.list_files(&self.directory) {
            Ok(files) => {
                for file in files {
                    if file.starts_with("backup_") {
                        backups.push(file);
                    }
                }
                Ok(backups)
            }
            Err(error) => {
                Err(error.to_string())
            }
        }
    }

    pub fn restore(&self, index : u8) -> Result<(), String> {
        match self.list() {
            Ok(backups) => {
                if(index >= backups.len() as u8) {
                    Err("Index out of range".to_string())
                }else {
                    let backup_path = format!("{}/{}", self.directory, backups[index as usize]);
                    match self.filesystem.copy_file(&backup_path, &self.path) {
                        Ok(_) => Ok(()),
                        Err(e) => Err(format!("Failed to restore backup index '{}' : {}", index, e)),   
                    }
                }
            }
            Err(error) => {
                Err(format!("Unable to retrieve backup list : {error}"))   
            }
        }
    }
}
