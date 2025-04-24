# Loaders

Loaders are transformations that are applied to the source code of a module. They allow you to pre-process files as you import or "load" them. This page documents the available loaders and how to create your own.

## Built-in Loaders

mini-rspack comes with a few built-in loaders:

### file-loader

The `file-loader` emits the file into the output directory and returns the public URL.

```javascript
module: {
  rules: [
    {
      test: '\\.png$',
      use: [
        path.resolve(__dirname, './loaders/file-loader.js')
      ]
    }
  ]
}
```

## Example Loaders

### babel-loader

The `babel-loader` transforms ES6+ code to ES5.

```javascript
module: {
  rules: [
    {
      test: '\\.js$',
      use: [
        path.resolve(__dirname, './loaders/babel-loader.js')
      ]
    }
  ]
}
```

### css-loader

The `css-loader` interprets `@import` and `url()` like `import/require()` and will resolve them.

```javascript
module: {
  rules: [
    {
      test: '\\.css$',
      use: [
        path.resolve(__dirname, './loaders/css-loader.js')
      ]
    }
  ]
}
```

### json-loader

The `json-loader` loads JSON files.

```javascript
module: {
  rules: [
    {
      test: '\\.json$',
      use: [
        path.resolve(__dirname, './loaders/json-loader.js')
      ]
    }
  ]
}
```

### html-loader

The `html-loader` exports HTML as string.

```javascript
module: {
  rules: [
    {
      test: '\\.html$',
      use: [
        path.resolve(__dirname, './loaders/html-loader.js')
      ]
    }
  ]
}
```

## Creating Custom Loaders

You can create your own loaders by creating a JavaScript function that takes the source code as input and returns the transformed source code.

### Loader Structure

```javascript
module.exports = function(source, name, modulePath) {
  // Transform the source code
  const transformedSource = source.replace('Hello', 'Hello from Loader');
  
  return transformedSource;
};
```

### Loader Parameters

- `source`: The content of the file
- `name`: The name of the module
- `modulePath`: The path of the module

### Example: babel-loader Implementation

Here's an example of how the `babel-loader` is implemented:

```javascript
/**
 * A simple babel-like loader that transforms ES6+ code to ES5
 */
module.exports = function(source) {
  // This is a simplified version that just does some basic transformations
  // In a real babel-loader, it would use the babel core to transform the code
  
  // Transform arrow functions to regular functions
  let transformed = source.replace(/const\s+(\w+)\s*=\s*\(([^)]*)\)\s*=>\s*{([^}]*)}/g, 'const $1 = function($2) {$3}');
  
  // Transform let/const to var
  transformed = transformed.replace(/let\s+/g, 'var ');
  transformed = transformed.replace(/const\s+/g, 'var ');
  
  // Transform template literals to string concatenation
  transformed = transformed.replace(/`([^`]*)\${([^}]*)}`/g, '"$1" + $2');
  
  // Add a comment to indicate the code was transformed
  return `// Transformed by babel-loader\n${transformed}`;
};
```

### Example: css-loader Implementation

Here's an example of how the `css-loader` is implemented:

```javascript
/**
 * A simple CSS loader that converts CSS to a JavaScript module
 */
module.exports = function(source, name, modulePath) {
  console.log(`CSS loader processing file: ${modulePath}`);
  
  // Escape quotes and newlines
  const escapedCSS = source
    .replace(/\\/g, '\\\\')
    .replace(/\'/g, '\\\'')
    .replace(/\n/g, '\\n');
  
  // Convert CSS to a JavaScript module that exports the CSS as a string
  const jsModule = `
    // CSS Module
    const css = '${escapedCSS}';
    
    // Create a style element
    const styleEl = document.createElement('style');
    styleEl.textContent = css;
    
    // Append to head when the module is imported
    document.head.appendChild(styleEl);
    
    // Export the CSS string
    module.exports = css;
  `;
  
  console.log('CSS transformation complete');
  return jsModule;
};
```

### Example: json-loader Implementation

Here's an example of how the `json-loader` is implemented:

```javascript
/**
 * A simple JSON loader that converts JSON to a JavaScript module
 */
module.exports = function(source, name, modulePath) {
  console.log(`JSON loader processing file: ${modulePath}`);
  
  try {
    // Parse the JSON to validate it
    const json = JSON.parse(source);
    
    // Convert JSON to a JavaScript module
    const jsModule = `module.exports = ${source};`;
    
    console.log('JSON transformation complete');
    return jsModule;
  } catch (error) {
    console.error(`Error parsing JSON in ${modulePath}: ${error.message}`);
    throw new Error(`Invalid JSON in ${modulePath}: ${error.message}`);
  }
};
```

### Example: html-loader Implementation

Here's an example of how the `html-loader` is implemented:

```javascript
/**
 * A simple HTML loader that converts HTML to a JavaScript module
 */
module.exports = function(source, name, modulePath) {
  console.log(`HTML loader processing file: ${modulePath}`);
  
  // Escape quotes and newlines
  const escapedHTML = source
    .replace(/\\/g, '\\\\')
    .replace(/\'/g, '\\\'')
    .replace(/\n/g, '\\n');
  
  // Convert HTML to a JavaScript module
  const jsModule = `
    // HTML Module
    const html = '${escapedHTML}';
    
    // Export the HTML string
    module.exports = html;
    
    // Export a function to create a DOM element
    module.exports.createDOM = function() {
      const template = document.createElement('template');
      template.innerHTML = html.trim();
      return template.content.firstChild;
    };
  `;
  
  console.log('HTML transformation complete');
  return jsModule;
};
```

## Chaining Loaders

You can chain multiple loaders to transform a file. The loaders are applied from right to left.

```javascript
module: {
  rules: [
    {
      test: '\\.css$',
      use: [
        path.resolve(__dirname, './loaders/style-loader.js'),
        path.resolve(__dirname, './loaders/css-loader.js')
      ]
    }
  ]
}
```

In this example, the file is first processed by the `css-loader`, and then the result is processed by the `style-loader`.

## Using Loaders

To use a loader, add it to the `module.rules` array in your configuration:

```javascript
const { createCompiler } = require('mini-rspack');
const compiler = createCompiler({
  // ...
  module: {
    rules: [
      {
        test: '\\.js$',
        use: [
          path.resolve(__dirname, './loaders/babel-loader.js')
        ]
      },
      {
        test: '\\.css$',
        use: [
          path.resolve(__dirname, './loaders/style-loader.js'),
          path.resolve(__dirname, './loaders/css-loader.js')
        ]
      }
    ]
  }
});
```

## Next Steps

- [Plugins](/guide/plugins): Learn about the available plugins
- [Configuration](/guide/configuration): Learn about all configuration options
- [Architecture](/guide/architecture): Understand the architecture of mini-rspack
