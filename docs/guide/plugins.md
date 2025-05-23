# Plugins

Plugins extend the functionality of mini-rspack by tapping into hooks at different stages of the compilation process. This page documents the available plugins and how to create your own.

## Built-in Plugins

mini-rspack comes with a few built-in plugins:

### EmitPlugin

The `EmitPlugin` generates an `assets.md` file with a list of all assets.

```javascript
plugins: [
  'EmitPlugin'
]
```

## Example Plugins

### HtmlWebpackPlugin

The `HtmlWebpackPlugin` generates an HTML file with the bundled JavaScript files included.

```javascript
plugins: [
  'HtmlWebpackPlugin'
]
```

### BannerPlugin

The `BannerPlugin` adds a banner to the top of each generated file.

```javascript
plugins: [
  'BannerPlugin'
]
```

### CleanPlugin

The `CleanPlugin` cleans the output directory before compilation.

```javascript
plugins: [
  'CleanPlugin'
]
```

## Creating Custom Plugins

You can create your own plugins by creating a JavaScript class with an `apply` method that takes a compiler instance.

### Plugin Structure

```javascript
class MyPlugin {
  constructor(options) {
    this.options = options || {};
  }

  apply(compiler) {
    // Tap into compiler hooks
    compiler.hooks.emit.tap('MyPlugin', (compilation) => {
      // Modify the compilation
      compilation.assets['my-file.txt'] = 'Generated by MyPlugin';
    });
  }
}

module.exports = MyPlugin;
```

### Available Hooks

mini-rspack provides several hooks that plugins can tap into:

- `run`: Called before the compilation starts
- `emit`: Called before emitting assets to the output directory
- `done`: Called when the compilation is complete

### Example: BannerPlugin Implementation

Here's an example of how the `BannerPlugin` is implemented:

```javascript
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
        
        // Skip if entryOnly is true and this is not an entry file
        if (this.options && this.options.entryOnly) {
          const isEntry = Object.keys(compilation.entries).some(entry => 
            filename.startsWith(entry) || filename === `${entry}.js`
          );
          
          if (!isEntry) {
            return;
          }
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

### Example: HtmlWebpackPlugin Implementation

Here's an example of how the `HtmlWebpackPlugin` is implemented:

```javascript
class HtmlWebpackPlugin {
  constructor(options = {}) {
    this.options = {
      title: 'Mini Rspack App',
      template: null,
      filename: 'index.html',
      ...options
    };
  }

  apply(compiler) {
    // Tap into the emit hook
    compiler.hooks.emit.tap('HtmlWebpackPlugin', (compilation) => {
      // Generate HTML content
      const html = this.generateHtml(compilation);
      
      // Add the HTML file to the assets
      compilation.assets[this.options.filename] = html;
    });
  }

  generateHtml(compilation) {
    // If a template is provided, use it
    if (this.options.template) {
      const fs = require('fs');
      let template = fs.readFileSync(this.options.template, 'utf8');
      
      // Replace placeholders
      template = template.replace('<!-- title -->', this.options.title);
      
      // Add script tags for all entry points
      const scripts = Object.keys(compilation.assets)
        .filter(asset => asset.endsWith('.js'))
        .map(asset => `<script src="${asset}"></script>`)
        .join('\n    ');
      
      return template.replace('<!-- scripts -->', scripts);
    }
    
    // Otherwise, generate a basic HTML file
    return `<!DOCTYPE html>
<html>
  <head>
    <meta charset="utf-8">
    <title>${this.options.title}</title>
  </head>
  <body>
    <div id="app"></div>
    ${Object.keys(compilation.assets)
      .filter(asset => asset.endsWith('.js'))
      .map(asset => `<script src="${asset}"></script>`)
      .join('\n    ')}
  </body>
</html>`;
  }
}

module.exports = HtmlWebpackPlugin;
```

### Example: CleanPlugin Implementation

Here's an example of how the `CleanPlugin` is implemented:

```javascript
class CleanPlugin {
  constructor(options = {}) {
    this.options = {
      paths: [],
      ...options
    };
  }

  apply(compiler) {
    // Tap into the beforeRun hook
    compiler.hooks.run.tap('CleanPlugin', () => {
      const fs = require('fs');
      const path = require('path');
      
      // Get the output path from compiler options
      const outputPath = compiler.options.output.path;
      
      // If no specific paths are provided, clean the entire output directory
      if (this.options.paths.length === 0) {
        this.cleanDirectory(outputPath);
      } else {
        // Otherwise, clean only the specified paths
        this.options.paths.forEach(relativePath => {
          const fullPath = path.join(outputPath, relativePath);
          this.cleanPath(fullPath);
        });
      }
    });
  }

  cleanDirectory(directory) {
    const fs = require('fs');
    const path = require('path');
    
    if (!fs.existsSync(directory)) {
      return;
    }
    
    const files = fs.readdirSync(directory);
    
    for (const file of files) {
      const fullPath = path.join(directory, file);
      this.cleanPath(fullPath);
    }
  }

  cleanPath(fullPath) {
    const fs = require('fs');
    
    if (fs.existsSync(fullPath)) {
      const stats = fs.statSync(fullPath);
      
      if (stats.isDirectory()) {
        // Recursively clean subdirectories
        this.cleanDirectory(fullPath);
        
        // Remove the directory itself
        fs.rmdirSync(fullPath);
      } else {
        // Remove the file
        fs.unlinkSync(fullPath);
      }
    }
  }
}

module.exports = CleanPlugin;
```

## Using Plugins

To use a plugin, add it to the `plugins` array in your configuration:

```javascript
const { createCompiler } = require('mini-rspack');
const compiler = createCompiler({
  // ...
  plugins: [
    'EmitPlugin',
    'HtmlWebpackPlugin',
    'BannerPlugin'
  ]
});
```

## Next Steps

- [Loaders](/guide/loaders): Learn about the available loaders
- [Configuration](/guide/configuration): Learn about all configuration options
- [Architecture](/guide/architecture): Understand the architecture of mini-rspack
