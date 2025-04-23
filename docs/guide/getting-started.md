# Getting Started

This guide will help you get started with mini-rspack.

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

## Basic Usage

Create a JavaScript file that uses mini-rspack:

```javascript
const path = require('path');
const { rspack } = require('mini-rspack');

// Create rspack options
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
  }
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
```

## Watch Mode

mini-rspack supports watch mode, which automatically rebuilds when files change:

```javascript
// Enable watch mode in options
const rspackOptions = {
  // ...
  watch: true,
  // ...
};

// Create a compiler instance
const compiler = rspack(rspackOptions);

// Run in watch mode
compiler.watch((err, stats) => {
  if (err) {
    console.error('Watch compilation failed:', err);
    return;
  }
  console.log('Watch compilation successful!');
});
```

## Next Steps

- [Configuration](/guide/configuration): Learn about all configuration options
- [Plugins](/guide/plugins): Extend mini-rspack with plugins
- [Loaders](/guide/loaders): Transform module content with loaders
