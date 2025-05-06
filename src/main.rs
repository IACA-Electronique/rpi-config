mod clap_config;
mod config_file;
mod file_reader;
mod file_writer;
mod preset_manager;

use clap::Parser;
use clap_config::{Cli, Commands, PresetCommands};
use std::process;

use config_file::ConfigFile;
use file_reader::FileReader;
use file_writer::FileWriter;
use preset_manager::PresetManager;

const PRESETS_DIR: &str = "./presets";

fn main() {
    let cli = Cli::parse();
    let mut file = ConfigFile::new();

    load_config_or_exit(&mut file, &cli.file);
    handle_cli(&mut file, cli);
}

fn handle_cli(file: &mut ConfigFile, cli : Cli) {
    let path = &cli.file;
    match cli.command {
        Commands::Set {
            section,
            param,
            value,
        } => match file.set(&section, &param, &value) {
            Ok(_) => {
                log(&format!(
                    "Successfully set {param} = {value} in section [{section}]"
                ));
                save_config(file, &path);
            }
            Err(e) => error(&format!("Error setting value: {e}")),
        },
        Commands::Del { section, param } => {
            match file.delete(&section, &param) {
                Ok(_) => {
                    log(&format!("Successfully deleted {param} from section [{section}]"));
                    save_config(file, &path)
                }
                Err(e) => error(&format!("Error deleting value: {e}")),
            }
        }
        Commands::Preset { action } => match action {
            PresetCommands::Load { preset_name } => {
                let preset_manager = PresetManager::new(PRESETS_DIR, &path);
                match preset_manager.load(&preset_name) {
                    Ok(_) => {
                        log(&format!("Successfully loaded preset: {preset_name}"));
                    }
                    Err(e) => error(&format!("Error loading preset '{preset_name}' : {e}")),
                }
            }
            PresetCommands::Backup => {
                log("Creating backup of current configuration");
            }
        },
    }
}

fn save_config(file: &mut ConfigFile, path: &str) {
    let writer = FileWriter::new(path);
    match writer.write(&file.to_string()) {
        Ok(_) => log("Config file saved.\n"),
        Err(e) => {
            error(&format!("Unable to write config file to '{path}': {e}"));
            process::exit(1);
        }
    }
}

fn load_config_or_exit(file: &mut ConfigFile, path : &str) {
    let reader = FileReader::new(path);

    match reader.read() {
        Ok(content) => {
            file.load(&content);
            log("✅ Config file loaded.\n\n")
        }
        Err(e) => {
            error(&format!("Unable to read config file: {e}"));
            process::exit(1);
        }
    }
}

fn log(message: &str) {
    println!("{message}");
}

fn error(message: &str) {
    eprintln!("{message}");
}
