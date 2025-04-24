# Testing

mini-rspack has a comprehensive test suite that covers both the Rust core and the JavaScript API. This page explains the testing strategy and how to run the tests.

## Testing Strategy

The testing strategy for mini-rspack follows these principles:

1. **Unit Tests**: Test individual components in isolation
2. **Integration Tests**: Test how components work together
3. **End-to-End Tests**: Test the entire bundling process
4. **Rust and JavaScript**: Test both the Rust core and the JavaScript API

## Test Directory Structure

The tests are organized in the following directory structure:

```
tests/
├── js/              # JavaScript tests
│   ├── basic.test.js
│   ├── loaders.test.js
│   ├── modules.test.js
│   ├── plugins.test.js
│   └── jest.config.js
└── rust/            # Rust tests
    ├── main.rs
    ├── utils_test.rs
    ├── module_test.rs
    ├── compilation_test.rs
    ├── loader_test.rs
    ├── plugin_test.rs
    ├── compiler_test.rs
    ├── hook_test.rs
    └── lib_test.rs
```

## Rust Tests

The Rust tests use the standard Rust testing framework. Each module has its own test file that tests the functionality of that module.

### Running Rust Tests

To run the Rust tests, use the following command:

```bash
npm run test:rust
```

This will run all the Rust tests and show the results.

### Example Rust Test

Here's an example of a Rust test for the `Module` struct:

```rust
#[cfg(test)]
mod module_tests {
    use mini_rspack::module::{Module, Dependency};

    #[test]
    fn test_module_creation() {
        let module = Module {
            id: "./src/test.js".to_string(),
            name: "test".to_string(),
            source: "console.log('test');".to_string(),
            dependencies: vec![
                Dependency {
                    dep_module_id: "./dep1.js".to_string(),
                    dep_module_path: "./src/dep1.js".to_string(),
                },
                Dependency {
                    dep_module_id: "./dep2.js".to_string(),
                    dep_module_path: "./src/dep2.js".to_string(),
                },
            ],
        };
        
        assert_eq!(module.id, "./src/test.js");
        assert_eq!(module.name, "test");
        assert_eq!(module.source, "console.log('test');");
        assert_eq!(module.dependencies.len(), 2);
        assert_eq!(module.dependencies[0].dep_module_id, "./dep1.js");
        assert_eq!(module.dependencies[1].dep_module_id, "./dep2.js");
    }

    #[test]
    fn test_module_new() {
        let module = Module::new("./src/test.js".to_string(), "test".to_string());
        
        assert_eq!(module.id, "./src/test.js");
        assert_eq!(module.name, "test");
        assert_eq!(module.dependencies.len(), 0);
        assert_eq!(module.source, "");
    }
}
```

## JavaScript Tests

The JavaScript tests use Jest as the testing framework. They test the JavaScript API and how it interacts with the Rust core.

### Running JavaScript Tests

To run the JavaScript tests, use the following command:

```bash
npm run test:js
```

This will run all the JavaScript tests and show the results.

### Example JavaScript Test

Here's an example of a JavaScript test for the basic functionality:

```javascript
const assert = require('assert');
const path = require('path');
const fs = require('fs');
const { createCompiler } = require('../..');

describe('Basic Functionality', () => {
  const outputPath = path.resolve(__dirname, '../../dist-test');
  
  // Clean up output directory before tests
  beforeAll(() => {
    if (fs.existsSync(outputPath)) {
      fs.rmSync(outputPath, { recursive: true, force: true });
    }
    fs.mkdirSync(outputPath, { recursive: true });
  });
  
  // Clean up output directory after tests
  afterAll(() => {
    if (fs.existsSync(outputPath)) {
      fs.rmSync(outputPath, { recursive: true, force: true });
    }
  });
  
  test('should compile a simple module', (done) => {
    // Create a simple entry file
    const entryPath = path.resolve(outputPath, 'entry.js');
    fs.writeFileSync(entryPath, 'module.exports = "Hello, World!";');
    
    // Make sure the output directory exists
    if (!fs.existsSync(outputPath)) {
      fs.mkdirSync(outputPath, { recursive: true });
    }
    
    // Create compiler
    const compiler = createCompiler({
      context: outputPath,
      entry: {
        entries: {
          main: './entry.js'
        }
      },
      output: {
        path: outputPath,
        filename: '[name].bundle.js'
      }
    });
    
    // Run compiler
    compiler.run((err, stats) => {
      // Check for errors
      expect(err).toBeNull();
      expect(stats).toBeDefined();
      
      // Check output file exists
      const outputFile = path.resolve(outputPath, 'main.bundle.js');
      expect(fs.existsSync(outputFile)).toBe(true);
      
      // Check output file content
      const content = fs.readFileSync(outputFile, 'utf-8');
      expect(content).toContain('Hello, World!');
      
      done();
    });
  });
});
```

## Running All Tests

To run both the Rust and JavaScript tests, use the following command:

```bash
npm run test:all
```

This will run all the tests and show the results.

## Test Coverage

The test suite aims to cover the following aspects of mini-rspack:

1. **Module Parsing**: Test that modules are correctly parsed and dependencies are extracted
2. **Module Resolution**: Test that module paths are correctly resolved
3. **Loader System**: Test that loaders are correctly applied to modules
4. **Plugin System**: Test that plugins can modify the compilation process
5. **Code Generation**: Test that the generated code correctly bundles the modules
6. **Error Handling**: Test that errors are correctly reported

## Adding New Tests

When adding new features to mini-rspack, it's important to add tests for those features. Here's how to add new tests:

### Adding Rust Tests

1. Create a new test file in the `tests/rust` directory
2. Add the test module and test functions
3. Add the test module to `tests/rust/main.rs`

### Adding JavaScript Tests

1. Create a new test file in the `tests/js` directory
2. Add the test suite and test cases
3. Make sure the test file is included in the Jest configuration

## Continuous Integration

mini-rspack uses GitHub Actions for continuous integration. The CI pipeline runs all the tests on each pull request and push to the main branch.

## Next Steps

- [Performance](/advanced/performance): Explore performance optimizations
- [API Overview](/api/): Explore the API documentation
