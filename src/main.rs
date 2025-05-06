mod clap_config;
mod config_file;
mod file_reader;
mod file_writer;

use clap::Parser;
use clap_config::{Cli, Commands, PresetCommands};
use std::process;

use config_file::ConfigFile;
use file_reader::FileReader;
use file_writer::FileWriter;

const CONFIG_FILE: &str = "/boot/firmware/config.txt";

fn main() {
    let mut file = ConfigFile::new();

    load_config_or_exit(&mut file);
    handle_cli(&mut file);
}

fn handle_cli(file: &mut ConfigFile) {
    let cli = Cli::parse();
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
                save_config(file);
            }
            Err(e) => error(&format!("Error setting value: {e}")),
        },
        Commands::Del { section, param } => {
            match file.delete(&section, &param) {
                Ok(_) => {
                    log(&format!("Successfully deleted {param} from section [{section}]"));
                    save_config(file)
                }
                Err(e) => error(&format!("Error deleting value: {e}")),
            }
        }
        Commands::Preset { action } => match action {
            PresetCommands::Load { preset_name } => {
                log(&format!("Loading preset: {preset_name}"));
            }
            PresetCommands::Backup => {
                log("Creating backup of current configuration");
            }
        },
    }
}

fn save_config(file: &mut ConfigFile) {
    let writer = FileWriter::new(CONFIG_FILE);
    match writer.write(&file.to_string()) {
        Ok(_) => log("Config file saved.\n"),
        Err(e) => {
            error(&format!("Unable to write config file: {e}"));
            process::exit(1);
        }
    }
}

fn load_config_or_exit(file: &mut ConfigFile) {
    let reader = FileReader::new(CONFIG_FILE);

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
