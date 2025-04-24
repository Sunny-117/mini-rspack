---
layout: home
hero:
  name: mini-rspack
  text: A simplified Rspack bundler using Rust
  tagline: High-performance JavaScript bundling with Rust and Node.js
  image:
    src: https://assets.rspack.dev/rspack/rspack-logo.svg
    alt: mini-rspack
  actions:
    - theme: brand
      text: Get Started
      link: /guide/
    - theme: alt
      text: View on GitHub
      link: https://github.com/Sunny-117/mini-rspack

features:
  - icon: 🚀
    title: High Performance
    details: Built with Rust for improved performance compared to JavaScript-based bundlers
  - icon: 🔌
    title: Webpack Compatible
    details: Familiar API for webpack users, making migration easy
  - icon: 🧩
    title: Extensible
    details: Support for plugins and loaders to extend functionality
  - icon: 🔄
    title: Watch Mode
    details: Automatically rebuild when files change
---

## What is mini-rspack?

mini-rspack is a simplified implementation of Rspack bundler using Rust and Node.js bindings. It demonstrates how to create a JavaScript bundler with Rust for improved performance. This project serves as an educational resource for understanding modern bundler architecture and the internals of tools like webpack and rspack.

## Quick Start

```bash
# Clone the repository
git clone https://github.com/Sunny-117/mini-rspack.git
cd mini-rspack

# Install dependencies
npm install

# Build the Rust code
npm run build

# Create a configuration file
const { createCompiler } = require('mini-rspack');
const compiler = createCompiler(options);

# Run the compiler
compiler.run((err, stats) => {
  console.log('Compilation successful!');
});
```

## Features

- **Webpack-compatible API**: Familiar interface for webpack users
- **Support for CommonJS and ES modules**: Handle both module formats
- **Entry point processing**: Support for multiple entry points
- **Module resolution**: Resolve dependencies between modules with custom resolver
- **Plugin system with hooks**: Extend functionality with plugins (similar to Tapable)
- **Loader system**: Transform module content with loaders
- **Watch mode**: Automatically rebuild when files change
- **Comprehensive test suite**: Tests in both Rust and JavaScript

## Technical Implementation

- **Rust Core**: Core bundling logic implemented in Rust for performance
- **napi-rs**: Node.js native modules from Rust code with proper bindings
- **Module Parsing**: Regex and basic AST analysis to extract dependencies
- **Dependency Resolution**: Path resolution similar to Node.js module resolution
- **Code Generation**: JavaScript bundle with proper module wrapping and runtime
- **Hook System**: Hook-based plugin system similar to webpack's Tapable
