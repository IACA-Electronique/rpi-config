use ini_config::low_level::filesystem_manager::FileSystemManager;
use ini_config::backup_manager::BackupManager;
#[cfg(test)]
mod tests {
    use super::*;
    use mockall::predicate::*;
    use mockall::mock;

    mock! {
        FileSystemManager {}
        impl FileSystemManager for FileSystemManager {
            fn list_files(&self, dir: &str) -> Result<Vec<String>, String>;
            fn copy_file(&self, src: &str, dst: &str) -> Result<(), String>;
            fn read_file(&self, path: &str) -> Result<String, String>;
            fn delete_file(&self, path: &str) -> Result<(), String>;
            fn exists(&self, path: &str) -> Result<bool, String>;
            fn create_dir_if_not_exists(&self, path: &str) -> Result<(), String>;
        }
    }

    #[test]
    fn test_create_backup_success() {
        let mut mock = MockFileSystemManager::new();

        mock.expect_copy_file()
            .withf(|src: &str, dst: &str| {
                src == "/test/file.txt" &&
                    dst.starts_with("/backups/backup_") &&
                    dst.ends_with(".bak")
            })
            .times(1)
            .returning(|_, _| Ok(()));

        mock.expect_create_dir_if_not_exists().withf(|path : &str| {
            path == "/backups"
        }).times(1).returning(|_| {
            Ok(())
        });

        let backup_manager = BackupManager::new(
            "/test/file.txt",
            "/backups",
            Box::new(mock)
        );

        assert!(backup_manager.create().is_ok());
    }

    #[test]
    fn test_create_backup_failure() {
        let mut mock = MockFileSystemManager::new();

        mock.expect_copy_file()
            .with(always(), always())
            .times(1)
            .returning(|_, _| Err("Mock file copy error".to_string()));

        mock.expect_create_dir_if_not_exists().withf(|path : &str| {
            path == "/backups"
        }).times(1).returning(|_| {
            Ok(())
        });
        
        let backup_manager = BackupManager::new(
            "/test/file.txt",
            "/backups",
            Box::new(mock)
        );

        let result = backup_manager.create();
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Failed to create backup"));
    }

    #[test]
    fn test_list_backups_success() {
        let mut mock = MockFileSystemManager::new();

        mock.expect_list_files()
            .with(eq("/backups"))
            .times(1)
            .returning(|_| Ok(vec![
                "backup_20240101_120000_42.bak".to_string(),
                "backup_20240101_120001_43.bak".to_string(),
                "other_file.txt".to_string()
            ]));

        let backup_manager = BackupManager::new(
            "/test/file.txt",
            "/backups",
            Box::new(mock)
        );

        let result = backup_manager.list().unwrap();
        assert_eq!(result.len(), 2);
        assert!(result.iter().all(|f| f.starts_with("backup_")));
    }

    #[test]
    fn test_list_backups_failure() {
        let mut mock = MockFileSystemManager::new();

        mock.expect_list_files()
            .with(eq("/backups"))
            .times(1)
            .returning(|_| Err("Mock list error".to_string()));

        let backup_manager = BackupManager::new(
            "/test/file.txt",
            "/backups",
            Box::new(mock)
        );

        assert!(backup_manager.list().is_err());
    }

    #[test]
    fn test_restore_success() {
        let mut mock = MockFileSystemManager::new();

        // Setup list_files expectation
        mock.expect_list_files()
            .with(eq("/backups"))
            .times(1)
            .returning(|_| Ok(vec![
                "backup_20240101_120000_42.bak".to_string(),
                "backup_20240101_120001_43.bak".to_string()
            ]));

        // Setup copy_file expectation
        mock.expect_copy_file()
            .withf(|src: &str, dst: &str| {
                src == "/backups/backup_20240101_120000_42.bak" &&
                    dst == "/test/file.txt"
            })
            .times(1)
            .returning(|_, _| Ok(()));

        let backup_manager = BackupManager::new(
            "/test/file.txt",
            "/backups",
            Box::new(mock)
        );

        assert!(backup_manager.restore(0).is_ok());
    }

    #[test]
    fn test_restore_invalid_index() {
        let mut mock = MockFileSystemManager::new();

        mock.expect_list_files()
            .with(eq("/backups"))
            .times(1)
            .returning(|_| Ok(vec![
                "backup_20240101_120000_42.bak".to_string()
            ]));

        let backup_manager = BackupManager::new(
            "/test/file.txt",
            "/backups",
            Box::new(mock)
        );

        let result = backup_manager.restore(1);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Index out of range");
    }

    #[test]
    fn test_restore_list_failure() {
        let mut mock = MockFileSystemManager::new();

        mock.expect_list_files()
            .with(eq("/backups"))
            .times(1)
            .returning(|_| Err("Mock list error".to_string()));

        let backup_manager = BackupManager::new(
            "/test/file.txt",
            "/backups",
            Box::new(mock)
        );

        let result = backup_manager.restore(0);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Unable to retrieve backup list"));
    }

    #[test]
    fn test_restore_copy_failure() {
        let mut mock = MockFileSystemManager::new();

        mock.expect_list_files()
            .with(eq("/backups"))
            .times(1)
            .returning(|_| Ok(vec![
                "backup_20240101_120000_42.bak".to_string()
            ]));

        mock.expect_copy_file()
            .with(always(), always())
            .times(1)
            .returning(|_, _| Err("Mock copy error".to_string()));

        let backup_manager = BackupManager::new(
            "/test/file.txt",
            "/backups",
            Box::new(mock)
        );

        let result = backup_manager.restore(0);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Failed to restore backup index"));
    }
}
