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

#[test]
fn test_to_string_empty_config() {
    let config = ConfigFile::new();
    assert_eq!(config.to_string(), "");
}

#[test]
fn test_to_string_single_section() {
    let mut config = ConfigFile::new();
    config.load("[test_section]\nkey=value\n");
    assert_eq!(config.to_string(), "[test_section]\nkey=value\n");
}

#[test]
fn test_to_string_multiple_sections() {
    let mut config = ConfigFile::new();
    config.load("[section1]\nkey1=value1\n[section2]\nkey2=value2\n");
    let result = config.to_string();
    // Since the order of sections is not guaranteed by the HashMap in rust-ini,
    // we need to check for both possible orderings
    let expected1 = "[section1]\nkey1=value1\n[section2]\nkey2=value2\n";
    let expected2 = "[section2]\nkey2=value2\n[section1]\nkey1=value1\n";
    assert!(result == expected1 || result == expected2);
}

#[test]
fn test_to_string_multiple_keys_in_section() {
    let mut config = ConfigFile::new();
    config.load("[section]\nkey1=value1\nkey2=value2\n");
    let result = config.to_string();
    // Since the order of keys is not guaranteed by the HashMap in rust-ini,
    // we need to check for both possible orderings
    let expected1 = "[section]\nkey1=value1\nkey2=value2\n";
    let expected2 = "[section]\nkey2=value2\nkey1=value1\n";
    assert!(result == expected1 || result == expected2);
}
