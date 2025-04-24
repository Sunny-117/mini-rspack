# Performance

mini-rspack is designed with performance in mind. This page explains the performance optimizations and how to measure and improve performance.

## Performance Advantages of Rust

Using Rust for the core functionality provides several performance advantages:

1. **Zero-Cost Abstractions**: Rust's abstractions have no runtime overhead
2. **No Garbage Collection**: Rust's ownership model eliminates the need for garbage collection
3. **Predictable Performance**: Rust's performance characteristics are more predictable than JavaScript
4. **Concurrency**: Rust's type system and ownership model make it easier to write concurrent code safely

## Performance Optimizations

mini-rspack includes several performance optimizations:

### 1. Minimal Copying

Data is passed between Rust and JavaScript with minimal copying. This is achieved by using napi-rs's efficient data conversion mechanisms.

```rust
// Example of efficient data conversion with napi-rs
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

### 2. Efficient Module Resolution

The module resolution algorithm is optimized to minimize file system operations:

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

### 3. Caching

Module resolution results are cached to avoid redundant work:

```rust
pub fn resolve_module(&mut self, module_id: &str, context: &Path) -> Result<PathBuf> {
    // Check if the module is already resolved
    if let Some(path) = self.resolved_modules.get(module_id) {
        return Ok(path.clone());
    }

    // Resolve the module
    let module_path = self.resolve_module_path(module_id, context)?;

    // Cache the result
    self.resolved_modules.insert(module_id.to_string(), module_path.clone());

    Ok(module_path)
}
```

### 4. Simplified Parsing

mini-rspack uses regex patterns for module parsing instead of a full AST parser. While this approach is simpler, it's also more efficient for basic module analysis:

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

## Measuring Performance

To measure the performance of mini-rspack, you can use the following approaches:

### 1. Time Measurement

You can measure the time it takes to bundle a project:

```javascript
const { createCompiler } = require('mini-rspack');
const compiler = createCompiler(options);

console.time('Bundling');
compiler.run((err, stats) => {
  console.timeEnd('Bundling');
  console.log('Bundling completed in', stats.time, 'ms');
});
```

### 2. Memory Usage

You can measure the memory usage during bundling:

```javascript
const { createCompiler } = require('mini-rspack');
const compiler = createCompiler(options);

const memoryBefore = process.memoryUsage().heapUsed / 1024 / 1024;
console.log(`Memory usage before: ${memoryBefore} MB`);

compiler.run((err, stats) => {
  const memoryAfter = process.memoryUsage().heapUsed / 1024 / 1024;
  console.log(`Memory usage after: ${memoryAfter} MB`);
  console.log(`Memory usage difference: ${memoryAfter - memoryBefore} MB`);
});
```

### 3. Profiling

You can use the Node.js built-in profiler to profile the bundling process:

```bash
node --prof app.js
```

This will generate a log file that you can analyze with the `--prof-process` flag:

```bash
node --prof-process isolate-0xnnnnnnnnnnnn-v8.log > processed.txt
```

## Performance Comparison

To compare the performance of mini-rspack with other bundlers like webpack, you can use the following approach:

1. Create a sample project with a similar structure
2. Bundle the project with mini-rspack and measure the time and memory usage
3. Bundle the project with webpack and measure the time and memory usage
4. Compare the results

## Future Performance Improvements

There are several areas where mini-rspack's performance could be improved in the future:

1. **Parallel Processing**: Implement parallel processing of modules
2. **Incremental Compilation**: Only rebuild what has changed
3. **Better Caching**: Implement more sophisticated caching strategies
4. **AST Caching**: Cache parsed ASTs to avoid reparsing
5. **Memory Optimization**: Reduce memory usage during bundling

## Next Steps

- [Technical Implementation](/advanced/): Dive deeper into the technical details
- [Testing](/advanced/testing): Learn about the testing strategy
- [API Overview](/api/): Explore the API documentation
