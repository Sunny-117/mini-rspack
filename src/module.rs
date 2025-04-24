use napi_derive::napi;
use std::path::Path;
use serde::{Deserialize, Serialize};
use anyhow::Result;

#[napi(object)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Module {
    pub id: String,
    pub name: String,
    pub dependencies: Vec<Dependency>,
    pub source: String,
}

#[napi(object)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dependency {
    pub dep_module_id: String,
    pub dep_module_path: String,
}

impl Module {
    pub fn new(id: String, name: String) -> Self {
        Self {
            id,
            name,
            dependencies: Vec::new(),
            source: String::new(),
        }
    }

    // Parse the module source code and extract dependencies
    pub fn parse_dependencies(&mut self, source_code: &str, module_path: &Path, base_dir: &Path, resolve_extensions: &[String]) -> Result<()> {
        // For simplicity in this implementation, we'll just use a simple string search
        // to find require calls instead of fully parsing the AST

        // In a real implementation, we would use SWC to parse the code and extract require calls
        // by traversing the AST

        // Set the source code
        self.source = source_code.to_string();

        // Process CommonJS requires
        self.process_commonjs_requires(source_code, module_path, base_dir, resolve_extensions)?;
        
        // Process ESM imports
        self.process_esm_imports(source_code, module_path, base_dir, resolve_extensions)?;

        Ok(())
    }
    
    // Process CommonJS require statements
    fn process_commonjs_requires(&mut self, source_code: &str, module_path: &Path, base_dir: &Path, resolve_extensions: &[String]) -> Result<()> {
        // Simple regex to find require calls
        let require_regex = regex::Regex::new(r#"require\(['"](.+?)['"]\)"#).unwrap();

        for cap in require_regex.captures_iter(source_code) {
            if let Some(module_name) = cap.get(1) {
                let module_name = module_name.as_str();
                self.process_module_dependency(module_name, module_path, base_dir, resolve_extensions, "CommonJS")?;
            }
        }
        
        Ok(())
    }
    
    // Process ESM import statements
    fn process_esm_imports(&mut self, source_code: &str, module_path: &Path, base_dir: &Path, resolve_extensions: &[String]) -> Result<()> {
        // Static import statements
        let static_import_regex = regex::Regex::new(r#"import .* from ['"](.+?)['"]\s*;"#).unwrap();
        
        // Dynamic import statements
        let dynamic_import_regex = regex::Regex::new(r#"import\(['"](.+?)['"]\)"#).unwrap();
        
        // Re-export statements
        let reexport_regex = regex::Regex::new(r#"export\s+\*\s+from\s+['"](.+?)['"]\s*;"#).unwrap();

        // Process static imports
        for cap in static_import_regex.captures_iter(source_code) {
            if let Some(module_name) = cap.get(1) {
                self.process_module_dependency(module_name.as_str(), module_path, base_dir, resolve_extensions, "static ESM")?;
            }
        }
        
        // Process dynamic imports
        for cap in dynamic_import_regex.captures_iter(source_code) {
            if let Some(module_name) = cap.get(1) {
                self.process_module_dependency(module_name.as_str(), module_path, base_dir, resolve_extensions, "dynamic ESM")?;
            }
        }
        
        // Process re-exports
        for cap in reexport_regex.captures_iter(source_code) {
            if let Some(module_name) = cap.get(1) {
                self.process_module_dependency(module_name.as_str(), module_path, base_dir, resolve_extensions, "re-export ESM")?;
            }
        }
        
        Ok(())
    }
    
    // Helper function to process module dependencies
    fn process_module_dependency(&mut self, module_name: &str, module_path: &Path, base_dir: &Path, resolve_extensions: &[String], import_type: &str) -> Result<()> {
        println!("Found {} dependency: {}", import_type, module_name);

        // Resolve the module path
        let dirname = module_path.parent().unwrap();
        let dep_module_path = dirname.join(module_name);
        println!("Resolving path: {:?}", dep_module_path);

        // Try to resolve with extensions
        match crate::utils::try_extensions(&dep_module_path, resolve_extensions) {
            Ok(resolved_path) => {
                println!("Resolved to: {:?}", resolved_path);
                // Get the module ID (relative to base_dir)
                let dep_module_id = format!("./{}", pathdiff::diff_paths(&resolved_path, base_dir).unwrap().to_string_lossy());
                println!("Module ID: {}", dep_module_id);

                // Add the dependency
                self.dependencies.push(Dependency {
                    dep_module_id,
                    dep_module_path: resolved_path.to_string_lossy().to_string(),
                });
            },
            Err(err) => {
                eprintln!("Failed to resolve module {}: {}", module_name, err);
            }
        }
        
        Ok(())
    }
}
