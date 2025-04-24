# Architecture

mini-rspack follows a modular architecture similar to webpack, with components designed to work together to transform source code into bundled output. This page explains the high-level architecture and how the different components interact.

## Overview

The architecture of mini-rspack consists of the following main components:

1. **Compiler**: The main entry point that manages the compilation process
2. **Compilation**: Represents a single build of the application
3. **Module**: Represents a module in the dependency graph
4. **Loader**: Transforms module content
5. **Plugin**: Extends the functionality of the bundler
6. **Hook System**: Provides a way for plugins to tap into different stages of the compilation
7. **Resolver**: Resolves module paths and handles module resolution algorithms

## Component Interaction

Here's how these components interact during the bundling process:

1. The **Compiler** is created with a configuration object
2. When `compiler.run()` is called, it creates a new **Compilation** instance
3. The **Compilation** reads the entry points and starts building the dependency graph
4. For each module, it:
   - Resolves the module path using the **Resolver**
   - Reads the module content
   - Applies **Loaders** to transform the content
   - Parses the transformed content to extract dependencies
   - Adds the module to the dependency graph
5. Once all modules are processed, the **Compilation** creates chunks and generates assets
6. Throughout this process, **Hooks** are called, allowing **Plugins** to modify the behavior

## Rust and JavaScript Interaction

mini-rspack uses Rust for the core functionality and exposes a JavaScript API. Here's how the two languages interact:

1. The JavaScript API is defined in `index.js` and provides a webpack-like interface
2. The Rust code is compiled to a native Node.js module using napi-rs
3. The JavaScript API calls into the Rust code to perform the actual bundling
4. Callbacks and hooks allow JavaScript code (like plugins and loaders) to interact with the Rust core

## Code Structure

The codebase is organized as follows:

```
mini-rspack/
├── src/                 # Rust source code
│   ├── lib.rs           # Main library entry point
│   ├── compiler.rs      # Compiler implementation
│   ├── compilation.rs   # Compilation implementation
│   ├── module.rs        # Module implementation
│   ├── loader.rs        # Loader system
│   ├── loader_runner.rs # Loader runner implementation
│   ├── plugin.rs        # Plugin system
│   ├── hook.rs          # Hook system
│   └── utils.rs         # Utility functions
├── index.js             # JavaScript API
├── loaders/             # Example loaders
├── plugins/             # Example plugins
└── tests/               # Test files
    ├── js/              # JavaScript tests
    └── rust/            # Rust tests
```

## Compilation Process

The compilation process in mini-rspack follows these steps:

1. **Initialization**: Create a compiler instance with the provided configuration
2. **Entry Resolution**: Resolve the entry points specified in the configuration
3. **Module Processing**: For each module:
   - Resolve the module path
   - Read the module content
   - Apply loaders to transform the content
   - Parse the transformed content to extract dependencies
   - Add the module to the dependency graph
4. **Dependency Resolution**: Resolve dependencies for each module and repeat the module processing step
5. **Chunk Creation**: Group modules into chunks based on entry points and dynamic imports
6. **Asset Generation**: Generate assets (JavaScript files) from the chunks
7. **Plugin Processing**: Allow plugins to modify the generated assets
8. **Output**: Write the assets to the output directory

## Technical Implementation Details

### Module Parsing

mini-rspack uses regex patterns to extract dependencies from JavaScript modules:

```rust
// Extract CommonJS requires
let re_require = Regex::new(r#"require\(['"](.*?)['"]"#).unwrap();
for cap in re_require.captures_iter(source) {
    let dep = cap.get(1).unwrap().as_str();
    dependencies.push(dep.to_string());
}

// Extract ES module imports
let re_import = Regex::new(r#"import\s+.*?from\s+['"]([^'"]+)['"]"#).unwrap();
for cap in re_import.captures_iter(source) {
    let dep = cap.get(1).unwrap().as_str();
    dependencies.push(dep.to_string());
}

// Extract dynamic imports
let re_dynamic_import = Regex::new(r#"import\(['"]([^'"]+)['"]\)"#).unwrap();
for cap in re_dynamic_import.captures_iter(source) {
    let dep = cap.get(1).unwrap().as_str();
    dependencies.push(dep.to_string());
}
```

### Loader Runner

Loaders are JavaScript functions that transform module content. The loader runner executes these functions in sequence:

```rust
pub fn apply_loaders(source: &str, loaders: &Vec<Loader>, module_path: &Path) -> Result<String> {
    // If no loaders, return the source as is
    if loaders.is_empty() {
        return Ok(source.to_string());
    }

    // Create a temporary file for the loader runner
    let mut temp_file = tempfile::NamedTempFile::new()?;
    let loader_runner_path = temp_file.path();

    // Generate the loader runner code
    let loader_runner_code = generate_loader_runner(source, loaders, module_path)?;

    // Write the loader runner code to the temporary file
    std::fs::write(loader_runner_path, loader_runner_code)?;

    // Execute the loader runner
    let output = std::process::Command::new("node")
        .arg(loader_runner_path)
        .output()?;

    // Check if the execution was successful
    if !output.status.success() {
        let error_message = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow::anyhow!("Loader execution failed: {}", error_message));
    }

    // Get the transformed source from the output
    let transformed_source = String::from_utf8_lossy(&output.stdout).to_string();

    Ok(transformed_source)
}
```

### Hook System

The hook system allows plugins to tap into different stages of the compilation process:

```rust
pub struct SyncHook {
    pub name: String,
    pub taps: Vec<String>,
}

impl SyncHook {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            taps: Vec::new(),
        }
    }

    pub fn tap(&mut self, name: &str) {
        self.taps.push(name.to_string());
    }

    pub fn call(&self, args: Option<&mut HashMap<String, String>>) {
        // Call the tapped functions
        println!("Hook '{}' called with {} taps", self.name, self.taps.len());
        
        // For now, we'll just print the taps
        for tap in &self.taps {
            println!("  - Tap: {}", tap);
        }
    }
}
```

## Next Steps

- [Getting Started](/guide/getting-started): Learn how to install and use mini-rspack
- [API Overview](/api/): Explore the API documentation
- [Technical Implementation](/advanced/): Dive deeper into the technical details
