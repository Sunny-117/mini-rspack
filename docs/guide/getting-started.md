# Getting Started

This guide will help you get started with mini-rspack, a simplified implementation of webpack/rspack bundler using Rust and Node.js bindings.

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

## Project Structure

The project is organized as follows:

```
mini-rspack/
├── src/                 # Rust source code
│   ├── lib.rs           # Main library entry point
│   ├── compiler.rs      # Compiler implementation
│   ├── compilation.rs   # Compilation implementation
│   ├── module.rs        # Module implementation
│   ├── loader.rs        # Loader system
│   ├── loader_runner.rs # Loader runner implementation
│   ├── plugin.rs        # Plugin system
│   ├── hook.rs          # Hook system
│   └── utils.rs         # Utility functions
├── index.js             # JavaScript API
├── loaders/             # Example loaders
├── plugins/             # Example plugins
├── tests/               # Test files
│   ├── js/              # JavaScript tests
│   └── rust/            # Rust tests
└── docs/                # Documentation
```

## Basic Usage

Create a JavaScript file that uses mini-rspack:

```javascript
const path = require('path');
const { createCompiler } = require('mini-rspack');

// Create rspack options
const rspackOptions = {
  mode: 'production',
  devtool: false,
  watch: false,
  context: './',
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
        test: '\.js$',
        use: [
          path.resolve(__dirname, './loaders/babel-loader.js')
        ]
      }
    ]
  },
  plugins: [
    'EmitPlugin'
  ]
};

// Create a compiler instance
const compiler = createCompiler(rspackOptions);

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

### Understanding the Configuration

- **mode**: Sets the mode of the compilation ('development' or 'production')
- **devtool**: Controls source map generation
- **watch**: Enables watch mode for automatic rebuilds
- **context**: The base directory for resolving entry points
- **entry**: Defines the entry points of the application
- **output**: Configures how and where to output the bundled files
- **resolve**: Configures how modules are resolved
- **module**: Configures how different modules are treated
- **plugins**: Extends the functionality with plugins

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
const compiler = createCompiler(rspackOptions);

// Run in watch mode
compiler.watch((err, stats) => {
  if (err) {
    console.error('Watch compilation failed:', err);
    return;
  }
  console.log('Watch compilation successful!');
});
```

## Creating Custom Loaders

Loaders are JavaScript functions that transform module content. Here's an example of a simple loader that transforms JavaScript code:

```javascript
// loaders/my-loader.js
module.exports = function(source, name, modulePath) {
  console.log(`Processing ${modulePath} with my-loader`);

  // Transform the source code
  const transformedSource = source.replace('Hello', 'Hello from my-loader');

  return transformedSource;
};
```

To use this loader, add it to the module rules in your configuration:

```javascript
module: {
  rules: [
    {
      test: '\.js$',
      use: [
        path.resolve(__dirname, './loaders/my-loader.js')
      ]
    }
  ]
}
```

## Creating Custom Plugins

Plugins extend the functionality of mini-rspack by tapping into hooks. Here's an example of a simple plugin that adds a banner to the top of each generated file:

```javascript
// plugins/BannerPlugin.js
class BannerPlugin {
  constructor(options) {
    if (typeof options === 'string') {
      this.banner = options;
    } else {
      this.options = {
        banner: '',
        entryOnly: false,
        ...options
      };
      this.banner = this.options.banner;
    }
  }

  apply(compiler) {
    // Tap into the emit hook
    compiler.hooks.emit.tap('BannerPlugin', (compilation) => {
      // Get all assets
      const assets = compilation.assets;

      // Iterate through assets
      Object.keys(assets).forEach(filename => {
        // Skip if not a JavaScript file
        if (!filename.endsWith('.js')) {
          return;
        }

        // Get the original source
        const source = assets[filename];

        // Add the banner to the top
        const bannerComment = `/*!\n * ${this.banner}\n */\n`;

        // Replace the asset with the new content
        compilation.assets[filename] = bannerComment + source;
      });
    });
  }
}

module.exports = BannerPlugin;
```

To use this plugin, add it to the plugins array in your configuration:

```javascript
plugins: [
  'BannerPlugin'
]
```

## Next Steps

- [Configuration](/guide/configuration): Learn about all configuration options
- [Plugins](/guide/plugins): Extend mini-rspack with plugins
- [Loaders](/guide/loaders): Transform module content with loaders
