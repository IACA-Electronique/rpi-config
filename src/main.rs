pub mod config_file;

fn main() {
    let mut file = config_file::ConfigFile::new();
    file.load("[test_section]\nexisting_key = existing_value\n");

    match file.get("all", "dtoverlay") {
        Some(value) => println!("{}", value),
        None => println!("No value found"),
    }

    match file.set("all", "dtoverlay", "toto") {
        Ok(_) => {
            println!("Value updated.")
        }
        Err(msg) => {
            println!("Error: {}", msg)
        }
    }

    match file.get("all", "dtoverlay") {
        Some(value) => println!("{}", value),
        None => println!("No value found"),
    }
}
