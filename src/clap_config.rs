use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "rpi-config")]
#[command(about = "Configure /boot/firmware/config.txt from command line")]
pub struct Cli {
    #[arg(short, long, default_value = "/boot/firmware/config.txt")]
    pub file: String,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Set {
        #[arg(short, long, default_value = "all")]
        section: String,
        param: String,
        value: String,
    },
    Del {
        #[arg(short, long, default_value = "all")]
        section: String,
        param: String,
    },
    Preset {
        #[command(subcommand)]
        action: PresetCommands,
    },
}

#[derive(Subcommand)]
pub enum PresetCommands {
    Load { preset_name: String },
    Backup,
}
