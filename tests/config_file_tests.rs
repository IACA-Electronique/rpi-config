use iaca_os_rpi_config::config_file::ConfigFile; // adjust the path according to your crate name

#[test]
fn test_set_without_loading_returns_error() {
    let mut config = ConfigFile::new();
    let result = config.set("test_section", "test_key", "test_value");
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Configuration not loaded".to_string());
}

#[test]
fn test_set_with_loaded_config() {
    let test_content = "[test_section]\nexisting_key = existing_value\n";
    let mut config = ConfigFile::new();
    config.load(test_content);

    let result = config.set("test_section", "new_key", "new_value");
    assert!(result.is_ok());

    // Verify the value was set correctly
    let value = config.get("test_section", "new_key");
    assert_eq!(value, Some("new_value".to_string()));
}

#[test]
fn test_set_updates_existing_value() {
    let test_content = "[test_section]\nexisting_key = old_value\n";
    let mut config = ConfigFile::new();
    config.load(test_content);

    let result = config.set("test_section", "existing_key", "new_value");
    assert!(result.is_ok());

    // Verify the value was updated
    let value = config.get("test_section", "existing_key");
    assert_eq!(value, Some("new_value".to_string()));
}

#[test]
fn test_set_creates_new_section() {
    let test_content = "[existing_section]\nkey = value\n";
    let mut config = ConfigFile::new();
    config.load(test_content);

    let result = config.set("new_section", "new_key", "new_value");
    assert!(result.is_ok());

    // Verify the value was set in the new section
    let value = config.get("new_section", "new_key");
    assert_eq!(value, Some("new_value".to_string()));
}
