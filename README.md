# mini-rspack

mini-rspack is a simplified implementation of webpack bundler using Rust and Node.js bindings. It demonstrates how to create a JavaScript bundler with Rust for improved performance.

[English](README.md) | [中文](README_zh.md)

## Introduction

mini-rspack is a proof-of-concept JavaScript bundler that uses Rust for the core functionality and exposes a JavaScript API similar to webpack. It demonstrates how to:

- Create Node.js native modules with Rust using napi-rs
- Implement a simplified bundler architecture
- Provide a familiar webpack-like API

## Features

- Webpack-compatible API
- Entry point processing
- Basic module resolution
- Plugin system with hooks
- Loader support
- Watch mode

## Installation

```bash
# Clone the repository
git clone https://github.com/yourusername/mini-rspack.git
cd mini-rspack

# Install dependencies
npm install

# Build the Rust code
npm run build
```

## Usage

Create a configuration file similar to webpack:

```javascript
const path = require('path');
const { rspack } = require('mini-rspack');

const rspackOptions = {
  mode: 'production',
  entry: {
    entries: {
      main: './src/index.js'
    }
  },
  output: {
    path: path.resolve(__dirname, 'dist'),
    filename: '[name].js'
  },
  resolve: {
    extensions: ['.js', '.json']
  },
  module: {
    rules: [
      {
        test: '\\.js$',
        use: [
          path.resolve(__dirname, './loaders/my-loader.js')
        ]
      }
    ]
  },
  plugins: [
    'MyPlugin'
  ]
};

// Create a compiler instance
const compiler = rspack(rspackOptions);

// Run the compiler
compiler.run((err, stats) => {
  if (err) {
    console.error('Compilation failed:', err);
    return;
  }

  console.log('Compilation successful!');
  console.log('Stats:', JSON.stringify(stats, null, 2));
});

// Or use watch mode
compiler.watch((err, stats) => {
  if (err) {
    console.error('Watch compilation failed:', err);
    return;
  }
  console.log('Watch compilation successful!');
});
```

## Architecture

mini-rspack consists of the following components:

1. **Compiler**: The main entry point that manages the compilation process
2. **Compilation**: Represents a single build of the application
3. **Module**: Represents a module in the dependency graph
4. **Loader**: Transforms module content
5. **Plugin**: Extends the functionality of the bundler

## Development

```bash
# Build in debug mode
npm run build:debug

# Run tests
npm test
```

## License

MIT
