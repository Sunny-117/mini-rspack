# API Overview

mini-rspack provides a JavaScript API similar to webpack/rspack. This section documents the available APIs and their implementation details.

## createCompiler

The main function to create a compiler instance. This function is implemented in Rust and exposed to JavaScript through napi-rs bindings.

```javascript
const { createCompiler } = require('mini-rspack');
const compiler = createCompiler(options);
```

### Parameters

- `options` (Object): The rspack configuration options
  - `mode` (String): The mode of the compilation ('development' or 'production')
  - `devtool` (Boolean): Controls source map generation
  - `watch` (Boolean): Enables watch mode for automatic rebuilds
  - `context` (String): The base directory for resolving entry points
  - `entry` (Object): Defines the entry points of the application
  - `output` (Object): Configures how and where to output the bundled files
  - `resolve` (Object): Configures how modules are resolved
  - `module` (Object): Configures how different modules are treated
  - `plugins` (Array): Extends the functionality with plugins

### Returns

- `Compiler`: A compiler instance

### Implementation Details

The `createCompiler` function is implemented in Rust as follows:

```rust
#[napi]
pub fn create_compiler(options: RspackOptions) -> Compiler {
    Compiler::new(options)
}
```

It creates a new `Compiler` instance with the provided options. The options are parsed from JavaScript to Rust using napi-rs's serialization capabilities.

## Compiler

The `Compiler` is the main entry point of mini-rspack. It manages the compilation process and provides hooks for plugins to tap into. It is implemented in Rust and exposed to JavaScript through napi-rs bindings.

```javascript
const compiler = createCompiler(options);

// Run the compiler
compiler.run((err, stats) => {
  // ...
});

// Watch mode
compiler.watch((err, stats) => {
  // ...
});
```

### Methods

#### run(callback)

Runs the compiler once. This method is implemented in Rust and exposed to JavaScript through napi-rs bindings.

- `callback` (Function): A callback function that is called when the compilation is complete
  - `err` (Error): An error object if the compilation failed
  - `stats` (Stats): A stats object containing information about the compilation

**Implementation Details:**

```rust
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
```

#### watch(callback)

Runs the compiler in watch mode, which automatically rebuilds when files change. This method is implemented in Rust and exposed to JavaScript through napi-rs bindings.

- `callback` (Function): A callback function that is called when each compilation is complete
  - `err` (Error): An error object if the compilation failed
  - `stats` (Stats): A stats object containing information about the compilation

**Implementation Details:**

```rust
#[napi]
pub fn watch(&mut self, callback: JsFunction) -> Result<()> {
    // Create a threadsafe function from the callback
    let tsfn = callback.create_threadsafe_function(0, |ctx| {
        let mut data = ctx.value.unwrap();
        let err = ctx.env.get_undefined().unwrap();
        let stats = ctx.env.to_js_value(&data).unwrap();
        Ok(vec![err, stats])
    })?;

    // Start the watch process
    watch_compiler(self, tsfn);

    Ok(())
}
```

### Properties

#### hooks

The `hooks` property provides access to the compiler hooks. Hooks are implemented using a simple event system similar to webpack's Tapable.

```javascript
compiler.hooks.done.tap('MyPlugin', (stats) => {
  console.log('Compilation done!');
});
```

Available hooks:

- `run`: Called before the compilation starts
- `emit`: Called before emitting assets to the output directory
- `done`: Called when the compilation is complete

**Implementation Details:**

```rust
pub struct CompilerHooks {
    pub run: SyncHook<Option<&mut HashMap<String, String>>>,
    pub emit: SyncHook<Option<&mut HashMap<String, String>>>,
    pub done: SyncHook<Option<&mut HashMap<String, String>>>,
}

impl CompilerHooks {
    pub fn new() -> Self {
        Self {
            run: SyncHook::new("run"),
            emit: SyncHook::new("emit"),
            done: SyncHook::new("done"),
        }
    }
}
```

## Stats

The `Stats` object contains information about the compilation. It is generated in Rust and passed to JavaScript through napi-rs bindings.

```javascript
compiler.run((err, stats) => {
  console.log(stats.entries); // Array of entry points
  console.log(stats.chunks); // Array of chunks
  console.log(stats.modules); // Array of modules
  console.log(stats.files); // Array of output files
  console.log(stats.assets); // Array of assets
});
```

### Properties

- `entries` (Array): The entry points of the application
- `chunks` (Array): The chunks generated during compilation
- `modules` (Array): The modules processed during compilation
- `files` (Array): The output files generated during compilation
- `assets` (Object): The assets generated during compilation, with keys being the asset names and values being the asset content

**Implementation Details:**

```rust
#[napi(object)]
pub struct Stats {
  pub entries: Vec<String>,
  pub chunks: Vec<String>,
  pub modules: Vec<String>,
  pub files: Vec<String>,
  pub assets: HashMap<String, String>,
}
```

## Module

The `Module` represents a module in the dependency graph. It contains information about the module's source code, dependencies, and how it should be processed.

**Implementation Details:**

```rust
#[napi(object)]
pub struct Module {
  pub id: String,
  pub name: String,
  pub source: String,
  pub dependencies: Vec<Dependency>,
}

#[napi(object)]
pub struct Dependency {
  pub dep_module_id: String,
  pub dep_module_path: String,
}
```

## Loader

Loaders transform the content of modules. They are implemented as JavaScript functions that can be chained together.

```javascript
module.exports = function(source, name, modulePath) {
  // Transform the source code
  return transformedSource;
};
```

**Implementation Details:**

Loaders are executed using a loader runner that is implemented in Rust:

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
