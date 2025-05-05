mod config_file;
mod file_writer;

fn main() {
    let writer = file_writer::FileWriter::new("toto.txt");
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

    match writer.write(&file.to_string()) {
        Ok(_) => println!("File written"),
        Err(msg) => println!("Error: {}", msg)       
    }
}
