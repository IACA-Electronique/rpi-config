use std::fs;

pub struct PresetManager {
    target: String,
    presets_dir : String,
}


impl PresetManager {
    pub fn new(presets_dir : &str, src_path : &str) -> Self {
        Self {
            target: src_path.to_string(),
            presets_dir : presets_dir.to_string(),
        }
    }
    
    pub fn load(&self, name : &str) -> Result<(), String>{
        let preset_path = format!("{}/{}.txt", &self.presets_dir, &name);
        let preset_exists : bool;
        
        match fs::exists(&preset_path) {
            Ok(exists) => preset_exists = exists,
            Err(_) => preset_exists = false,
        }
        
        if preset_exists {
            match fs::copy(&preset_path, &self.target) {
                Ok(_) => Ok(()),
                Err(err) => Err(err.to_string()),
            }
        } else {
            Err(format!("Preset '{}' does not exist (looking here : '{}')", &name, &preset_path))
        }
    }
}