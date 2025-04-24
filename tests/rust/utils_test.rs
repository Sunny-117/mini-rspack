#[cfg(test)]
mod utils_tests {
    use std::path::{Path, PathBuf};
    use mini_rspack::utils::{to_unix_path, try_extensions};

    #[test]
    fn test_to_unix_path() {
        // Test with Windows path
        assert_eq!(to_unix_path("C:\\Users\\test\\file.js"), "C:/Users/test/file.js");
        
        // Test with Unix path (should remain unchanged)
        assert_eq!(to_unix_path("/Users/test/file.js"), "/Users/test/file.js");
        
        // Test with mixed path
        assert_eq!(to_unix_path("C:\\Users/test\\file.js"), "C:/Users/test/file.js");
    }

    #[test]
    fn test_try_extensions() {
        // Create a temporary directory for testing
        let temp_dir = tempfile::tempdir().unwrap();
        let temp_path = temp_dir.path();
        
        // Create test files
        let js_file = temp_path.join("test.js");
        let ts_file = temp_path.join("test.ts");
        let json_file = temp_path.join("test.json");
        
        std::fs::write(&js_file, "// JS file").unwrap();
        std::fs::write(&ts_file, "// TS file").unwrap();
        std::fs::write(&json_file, "{}").unwrap();
        
        // Test with existing file (no extension needed)
        let result = try_extensions(&js_file, &vec![".js".to_string(), ".ts".to_string()]);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), js_file);
        
        // Test with file that needs extension
        let base_path = temp_path.join("test");
        let result = try_extensions(&base_path, &vec![".js".to_string(), ".ts".to_string()]);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), js_file);
        
        // Test with file that needs second extension
        let base_path = temp_path.join("test");
        let result = try_extensions(&base_path, &vec![".jsx".to_string(), ".ts".to_string()]);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), ts_file);
        
        // Test with non-existent file
        let non_existent = temp_path.join("non_existent");
        let result = try_extensions(&non_existent, &vec![".js".to_string(), ".ts".to_string()]);
        assert!(result.is_err());
    }
}
