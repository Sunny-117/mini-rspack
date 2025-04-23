use napi_derive::napi;
use std::path::Path;
use anyhow::Result;
use regex::Regex;

#[napi(object)]
#[derive(Debug, Clone)]
pub struct Loader {
    pub path: String,
}

impl Loader {
    pub fn new(path: String) -> Self {
        Self { path }
    }

    // This is a placeholder for the actual loader implementation
    // In a real implementation, we would load the JS loader and execute it
    pub fn run(&self, source_code: &str, _name: &str, module_path: &str) -> Result<String> {
        // For now, we'll just log the loader execution
        println!("Running loader: {} on module: {}", self.path, module_path);

        // In a real implementation, we would execute the JS loader
        // For now, we'll just return the source code unchanged
        Ok(source_code.to_string())
    }
}

// Find matching loaders for a module
pub fn find_matching_loaders(
    module_path: &Path,
    rules: &[crate::RuleOptions],
) -> Vec<Loader> {
    let mut loaders = Vec::new();

    for rule in rules {
        // Parse the test regex
        if let Ok(regex) = Regex::new(&rule.test) {
            let path_str = module_path.to_string_lossy();

            // Check if the module path matches the rule
            if regex.is_match(&path_str) {
                // Add all loaders from the rule
                for loader_path in &rule.use_ {
                    loaders.push(Loader::new(loader_path.clone()));
                }
            }
        }
    }

    loaders
}

// Apply loaders to a module
pub fn apply_loaders(
    source_code: &str,
    loaders: &[Loader],
    name: &str,
    module_path: &str,
) -> Result<String> {
    let mut processed_code = source_code.to_string();

    // Apply loaders in reverse order (right to left)
    for loader in loaders.iter().rev() {
        processed_code = loader.run(&processed_code, name, module_path)?;
    }

    Ok(processed_code)
}
