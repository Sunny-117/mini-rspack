use napi_derive::napi;
use std::path::Path;
use std::collections::HashMap;
use std::fs;
use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::module::Module;
use crate::loader::{find_matching_loaders, apply_loaders};
use crate::utils::to_unix_path;
use crate::WebpackOptions;
use crate::plugin::SyncHook;

#[napi(object)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chunk {
    pub name: String,
    pub entry_module: Module,
    pub modules: Vec<Module>,
}

#[napi(object)]
#[derive(Debug, Clone)]
pub struct Compilation {
    pub options: WebpackOptions,
    pub entries: Vec<Chunk>,
    pub modules: Vec<Module>,
    pub chunks: Vec<Chunk>,
    pub assets: HashMap<String, String>,
    pub files: Vec<String>,
    pub hooks: CompilationHooks,
}

#[napi(object)]
#[derive(Debug, Clone)]
pub struct CompilationHooks {
    pub emit: SyncHook,
}

impl Compilation {
    pub fn new(options: WebpackOptions, hooks: CompilationHooks) -> Self {
        Self {
            options,
            entries: Vec::new(),
            modules: Vec::new(),
            chunks: Vec::new(),
            assets: HashMap::new(),
            files: Vec::new(),
            hooks,
        }
    }

    pub fn make(&mut self) -> Result<()> {
        // Process entry points
        let entries = self.options.entry.entries.clone();

        // Get the context directory
        let context = self.options.context.clone().unwrap_or_else(|| std::env::current_dir().unwrap().to_string_lossy().to_string());
        let base_dir = Path::new(&context);

        // Process each entry point
        for (entry_name, entry_path) in entries {
            // Get the absolute path to the entry file
            let entry_file_path = base_dir.join(entry_path);
            let entry_file_path = to_unix_path(&entry_file_path);

            // Build the entry module
            let entry_module = self.build_module(&entry_name, &entry_file_path, base_dir)?;

            // Create a chunk for this entry
            let chunk = Chunk {
                name: entry_name.clone(),
                entry_module: entry_module.clone(),
                modules: self.modules.clone().into_iter().filter(|m| m.name == *entry_name).collect(),
            };

            // Add the chunk to entries and chunks
            self.entries.push(chunk.clone());
            self.chunks.push(chunk);
        }

        // Generate assets from chunks
        for chunk in &self.chunks {
            let filename = self.options.output.filename.replace("[name]", &chunk.name);
            let source = crate::utils::generate_bundle_source(chunk);
            self.assets.insert(filename, source);
        }

        // Update files list
        self.files = self.assets.keys().cloned().collect();

        // Call the emit hook
        self.hooks.emit.call(Some(&mut self.assets));

        // Write files to disk
        let output_path = Path::new(&self.options.output.path);
        for (filename, content) in &self.assets {
            let file_path = output_path.join(filename);

            // Create parent directories if they don't exist
            if let Some(parent) = file_path.parent() {
                fs::create_dir_all(parent)?;
            }

            // Write the file
            fs::write(&file_path, content)?;
        }

        Ok(())
    }

    fn build_module(&mut self, name: &str, module_path: &str, base_dir: &Path) -> Result<Module> {
        // Read the module file
        let source_code = fs::read_to_string(module_path)?;

        // Find matching loaders
        let rules = match &self.options.module {
            Some(module_options) => match &module_options.rules {
                Some(rules) => rules.clone(),
                None => Vec::new(),
            },
            None => Vec::new(),
        };

        let loaders = find_matching_loaders(Path::new(module_path), &rules);

        // Apply loaders
        let processed_code = apply_loaders(&source_code, &loaders, name, module_path)?;

        // Create a module
        let module_id = format!("./{}", pathdiff::diff_paths(module_path, base_dir).unwrap().to_string_lossy());
        let mut module = Module::new(module_id, name.to_string());

        // Parse dependencies
        let resolve_extensions = match &self.options.resolve {
            Some(resolve) => match &resolve.extensions {
                Some(extensions) => extensions.clone(),
                None => vec![".js".to_string(), ".json".to_string()],
            },
            None => vec![".js".to_string(), ".json".to_string()],
        };

        module.parse_dependencies(&processed_code, Path::new(module_path), base_dir, &resolve_extensions)?;

        // Add the module to the modules list
        self.modules.push(module.clone());

        // Process dependencies
        for dependency in &module.dependencies {
            // Check if the module has already been processed
            let dep_module_id = &dependency.dep_module_id;
            if !self.modules.iter().any(|m| &m.id == dep_module_id) {
                // Build the dependency module
                let dep_module = self.build_module(name, &dependency.dep_module_path, base_dir)?;
                self.modules.push(dep_module);
            }
        }

        Ok(module)
    }
}
