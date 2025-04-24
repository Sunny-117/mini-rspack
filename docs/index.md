---
layout: home
hero:
  name: mini-rspack
  text: A simplified webpack bundler using Rust
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

mini-rspack is a simplified implementation of webpack bundler using Rust and Node.js bindings. It demonstrates how to create a JavaScript bundler with Rust for improved performance.

## Quick Start

```bash
# Install
npm install mini-rspack

# Create a configuration file
const { rspack } = require('mini-rspack');
const compiler = rspack(options);

# Run the compiler
compiler.run((err, stats) => {
  console.log('Compilation successful!');
});
```

## Features

- **Webpack-compatible API**: Familiar interface for webpack users
- **Entry point processing**: Support for multiple entry points
- **Basic module resolution**: Resolve dependencies between modules
- **Plugin system with hooks**: Extend functionality with plugins
- **Loader support**: Transform module content with loaders
- **Watch mode**: Automatically rebuild when files change
