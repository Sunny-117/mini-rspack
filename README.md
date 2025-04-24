# mini-rspack

mini-rspack is a simplified implementation of Rspack bundler using Rust and Node.js bindings. It demonstrates how to create a JavaScript bundler with Rust for improved performance.

[English](README.md) | [中文](README_zh.md)

## Introduction

mini-rspack is a proof-of-concept JavaScript bundler that uses Rust for the core functionality and exposes a JavaScript API similar to webpack. It demonstrates how to:

- Create Node.js native modules with Rust using napi-rs
- Implement a simplified bundler architecture
- Parse and analyze JavaScript modules
- Resolve module dependencies
- Transform code with loaders
- Extend functionality with plugins
- Generate bundled output
- Provide a familiar webpack-like API

## Features

- Webpack-compatible API with compiler.run() method
- Support for both CommonJS and ES modules
- Entry point processing with multiple entry support
- Module resolution with custom resolver
- Plugin system with hooks (similar to Tapable)
- Loader system for transforming module content
- Watch mode for development

## Installation

```bash
# Clone the repository
git clone https://github.com/Sunny-117/mini-rspack.git
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

1. **Compiler**: The main entry point that manages the compilation process. Implemented in Rust with JavaScript bindings.
2. **Compilation**: Represents a single build of the application. Manages the build process and holds the state of the compilation.
3. **Module**: Represents a module in the dependency graph. Contains information about the module's source code, dependencies, and how it should be processed.
4. **Loader**: Transforms module content. Implemented as JavaScript functions that can be chained together.
5. **Plugin**: Extends the functionality of the bundler. Uses a hook system to tap into different stages of the compilation process.
6. **Hook System**: Similar to webpack's Tapable, provides a way for plugins to tap into different stages of the compilation.
7. **Resolver**: Resolves module paths and handles module resolution algorithms.

### Technical Implementation

- **Rust Core**: The core bundling logic is implemented in Rust for performance.
- **napi-rs**: Used to create Node.js native modules from Rust code.
- **JavaScript API**: Exposes a webpack-like API for configuration and usage.
- **Module Parsing**: Uses regex and basic AST analysis to extract dependencies.
- **Dependency Resolution**: Implements path resolution similar to Node.js module resolution.
- **Code Generation**: Generates JavaScript bundle with proper module wrapping and runtime.

## Development

```bash
# Build in debug mode
npm run build:debug

# Run tests
npm test

```

## Documentation

Detailed documentation is available in the `docs` directory. You can build and view the documentation using the provided scripts:

### Preview Documentation Locally

```bash
# Make the script executable
chmod +x ./preview-docs.sh

# Run the preview script
./preview-docs.sh
```

This will build the documentation and start a local server at http://localhost:5173/mini-rspack/

### Deploy Documentation to GitHub Pages

```bash
# Make the script executable
chmod +x ./deploy-docs.sh

# Run the deployment script
./deploy-docs.sh
```

This script will:
1. Build the documentation
2. Create or update the gh-pages branch
3. Push the built documentation to the gh-pages branch
4. Return to your original branch

The documentation will be available at https://Sunny-117.github.io/mini-rspack/

## Educational Resources

This project is designed as an educational resource for learning about:

- Modern JavaScript bundler architecture
- Rust and JavaScript interoperability
- Module resolution algorithms
- Plugin and loader systems
- AST parsing and code transformation
- Performance optimization in build tools

## License

MIT
