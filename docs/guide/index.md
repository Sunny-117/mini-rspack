# Introduction

mini-rspack is a proof-of-concept JavaScript bundler that uses Rust for the core functionality and exposes a JavaScript API similar to webpack. It demonstrates how to create high-performance JavaScript tooling using Rust and Node.js native modules.

## Why mini-rspack?

Modern JavaScript bundlers like webpack are powerful but can be slow when processing large applications. By implementing the core functionality in Rust, mini-rspack aims to provide:

1. **Better Performance**: Rust's speed and memory efficiency can significantly improve bundling times
2. **Familiar API**: A webpack-compatible API makes it easy for developers to adopt
3. **Learning Resource**: A simplified implementation to understand bundler architecture

## Core Concepts

mini-rspack follows similar concepts to webpack:

### Compiler

The `Compiler` is the main entry point of mini-rspack. It manages the compilation process and provides hooks for plugins to tap into.

```javascript
const { webpack } = require('mini-rspack');
const compiler = webpack(options);
```

### Compilation

A `Compilation` represents a single build of the application. It contains the modules, chunks, and assets generated during the build.

### Module

A `Module` represents a module in the dependency graph. It contains information about the module's source code, dependencies, and how it should be processed.

### Loader

Loaders transform the content of modules. They can be used to process non-JavaScript files or to transform JavaScript code.

### Plugin

Plugins extend the functionality of mini-rspack. They can tap into hooks provided by the compiler and compilation to modify the build process.

## Next Steps

- [Getting Started](/guide/getting-started): Learn how to install and use mini-rspack
- [Configuration](/guide/configuration): Explore configuration options
- [Plugins](/guide/plugins): Learn how to use and create plugins
- [Loaders](/guide/loaders): Learn how to use and create loaders
