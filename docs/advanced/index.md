# Technical Implementation

This section provides a deep dive into the technical implementation details of mini-rspack. It's designed for developers who want to understand how the bundler works under the hood.

## Rust Core

mini-rspack uses Rust for its core functionality, providing several advantages:

1. **Performance**: Rust's zero-cost abstractions and memory safety without garbage collection make it ideal for performance-critical applications like bundlers.
2. **Memory Safety**: Rust's ownership model prevents common bugs like null pointer dereferences, use-after-free, and data races.
3. **Concurrency**: Rust's type system and ownership model make it easier to write concurrent code safely.

The Rust core is responsible for:

- Parsing and analyzing JavaScript modules
- Resolving module dependencies
- Managing the compilation process
- Generating bundled output

## Node.js Integration with napi-rs

mini-rspack uses [napi-rs](https://napi.rs/) to create Node.js native modules from Rust code. napi-rs provides:

1. **Type Safety**: Strong type checking between Rust and JavaScript
2. **Performance**: Minimal overhead when calling between JavaScript and Rust
3. **Memory Safety**: Automatic management of JavaScript objects in Rust

Here's an example of how napi-rs is used to expose Rust functions to JavaScript:

```rust
#[napi]
pub fn create_compiler(options: RspackOptions) -> Compiler {
    Compiler::new(options)
}

#[napi]
impl Compiler {
    #[napi]
    pub fn run(&mut self, callback: JsFunction) -> Result<()> {
        // Create a threadsafe function from the callback
        let tsfn = callback.create_threadsafe_function(0, |ctx| {
            let mut data = ctx.value.unwrap();
            let err = ctx.env.get_undefined().unwrap();
            let stats = ctx.env.to_js_value(&data).unwrap();
            Ok(vec![err, stats])
        })?;

        // Run the compilation process
        let result = run_compiler(self);
        match result {
            Ok(stats) => {
                // Call the callback with the stats
                tsfn.call(Ok(stats), ThreadsafeFunctionCallMode::Blocking);
            }
            Err(err) => {
                // Call the callback with the error
                let error = ctx.env.create_error(err.to_string()).unwrap();
                tsfn.call(Err(error), ThreadsafeFunctionCallMode::Blocking);
            }
        }

        Ok(())
    }
}
```

## Module Parsing

mini-rspack uses regex patterns to extract dependencies from JavaScript modules. While this approach is simpler than a full AST parser, it's sufficient for basic module analysis:

```rust
pub fn parse_dependencies(source: &str) -> Vec<String> {
    let mut dependencies = Vec::new();

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

    dependencies
}
```

## Dependency Resolution

mini-rspack implements a module resolution algorithm similar to Node.js:

```rust
pub fn try_extensions(path: &Path, extensions: &Vec<String>) -> Result<PathBuf> {
    // If the path exists as is, return it
    if path.exists() {
        return Ok(path.to_path_buf());
    }

    // Try adding each extension
    for ext in extensions {
        let path_with_ext = path.with_extension(ext.trim_start_matches('.'));
        if path_with_ext.exists() {
            return Ok(path_with_ext);
        }
    }

    // If no match is found, return an error
    Err(anyhow::anyhow!("Could not resolve module: {}", path.display()))
}
```

## Loader System

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

The loader runner generates JavaScript code that loads and executes each loader:

```rust
fn generate_loader_runner(source: &str, loaders: &Vec<Loader>, module_path: &Path) -> Result<String> {
    let module_path_str = module_path.to_string_lossy();
    let escaped_source = escape_js_string(source);
    
    let mut loader_requires = String::new();
    let mut loader_calls = String::new();
    
    for (i, loader) in loaders.iter().enumerate() {
        let loader_var = format!("loader{}", i);
        loader_requires.push_str(&format!("const {} = require('{}');\n", loader_var, loader.path));
        
        if i == 0 {
            loader_calls.push_str(&format!("let result = {}('{}', '{}', '{}');\n", 
                                          loader_var, escaped_source, "", module_path_str));
        } else {
            loader_calls.push_str(&format!("result = {}(result, '{}', '{}');\n", 
                                          loader_var, "", module_path_str));
        }
    }
    
    let runner_code = format!(
        r#"
        // Loader runner generated by mini-rspack
        {}
        
        // Execute loaders
        {}
        
        // Output the result
        console.log(result);
        "#,
        loader_requires,
        loader_calls
    );
    
    Ok(runner_code)
}
```

## Plugin System

The plugin system allows extending the functionality of mini-rspack. Plugins can tap into hooks at different stages of the compilation process:

```rust
pub trait Plugin {
    fn apply(&self, compiler: &mut Compiler);
    fn name(&self) -> &str;
}

pub trait CompilationPlugin {
    fn apply(&self, compilation: &mut Compilation);
    fn name(&self) -> &str;
}
```

Plugins are registered with the compiler and can modify the compilation process:

```rust
pub fn apply_plugins(&mut self) {
    if let Some(plugins) = &self.options.plugins {
        for plugin_name in plugins {
            match plugin_name.as_str() {
                "EmitPlugin" => {
                    let plugin = EmitPlugin;
                    Plugin::apply(&plugin, self);
                }
                // Add more built-in plugins here
                _ => {
                    // Try to load the plugin from the plugins directory
                    if let Ok(plugin) = load_plugin(plugin_name) {
                        Plugin::apply(&plugin, self);
                    }
                }
            }
        }
    }
}
```

## Hook System

The hook system provides a way for plugins to tap into different stages of the compilation process:

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

## Code Generation

mini-rspack generates JavaScript code that wraps each module in a function and provides a runtime to handle module loading:

```rust
fn generate_runtime() -> String {
    r#"
// mini-rspack runtime
var __webpack_modules__ = {};
var __webpack_module_cache__ = {};

// The require function
function __webpack_require__(moduleId) {
    // Check if module is in cache
    var cachedModule = __webpack_module_cache__[moduleId];
    if (cachedModule !== undefined) {
        return cachedModule.exports;
    }
    // Create a new module (and put it into the cache)
    var module = __webpack_module_cache__[moduleId] = {
        exports: {}
    };
    // Execute the module function
    __webpack_modules__[moduleId](module, module.exports, __webpack_require__);
    // Return the exports of the module
    return module.exports;
}

// expose the modules object
__webpack_require__.m = __webpack_modules__;
"#.to_string()
}

fn generate_module_wrapper(module: &Module) -> String {
    format!(
        r#"
__webpack_modules__["{}"] = function(module, exports, __webpack_require__) {{
{}
}};
"#,
        module.id,
        module.source
    )
}
```

## Performance Considerations

mini-rspack is designed with performance in mind:

1. **Rust Core**: The core functionality is implemented in Rust, which provides better performance than JavaScript.
2. **Minimal Copying**: Data is passed between Rust and JavaScript with minimal copying.
3. **Efficient Module Resolution**: The module resolution algorithm is optimized to minimize file system operations.
4. **Caching**: Module resolution results are cached to avoid redundant work.

## Next Steps

- [Testing](/advanced/testing): Learn about the testing strategy
- [Performance](/advanced/performance): Explore performance optimizations
