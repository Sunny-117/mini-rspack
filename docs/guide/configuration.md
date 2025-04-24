# Configuration

mini-rspack provides a webpack-compatible configuration API. This page documents the available configuration options.

## Configuration Object

The configuration object is passed to the `createCompiler` function:

```javascript
const { createCompiler } = require('mini-rspack');
const compiler = createCompiler({
  // Configuration options
});
```

## Basic Configuration

Here's a basic configuration example:

```javascript
const path = require('path');
const { createCompiler } = require('mini-rspack');

const compiler = createCompiler({
  mode: 'development',
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
        test: '\\.js$',
        use: [
          path.resolve(__dirname, './loaders/babel-loader.js')
        ]
      }
    ]
  },
  plugins: [
    'EmitPlugin'
  ]
});
```

## Configuration Options

### `mode`

- Type: `String`
- Default: `'production'`
- Possible values: `'development'`, `'production'`

Sets the mode of the compilation. This affects how the code is bundled and optimized.

```javascript
mode: 'development'
```

### `devtool`

- Type: `Boolean`
- Default: `false`

Controls source map generation.

```javascript
devtool: false
```

### `watch`

- Type: `Boolean`
- Default: `false`

Enables watch mode for automatic rebuilds when files change.

```javascript
watch: true
```

### `context`

- Type: `String`
- Default: `'./'`

The base directory for resolving entry points and loaders.

```javascript
context: path.resolve(__dirname, 'src')
```

### `entry`

- Type: `Object`
- Required: `true`

Defines the entry points of the application.

```javascript
entry: {
  entries: {
    main: './src/index.js',
    vendor: './src/vendor.js'
  }
}
```

### `output`

- Type: `Object`
- Required: `true`

Configures how and where to output the bundled files.

```javascript
output: {
  path: path.resolve(__dirname, 'dist'),
  filename: '[name].js'
}
```

#### `output.path`

- Type: `String`
- Required: `true`

The output directory for the bundled files.

```javascript
path: path.resolve(__dirname, 'dist')
```

#### `output.filename`

- Type: `String`
- Default: `'[name].js'`

The filename pattern for the bundled files. The `[name]` placeholder is replaced with the entry point name.

```javascript
filename: '[name].[contenthash].js'
```

### `resolve`

- Type: `Object`
- Optional

Configures how modules are resolved.

```javascript
resolve: {
  extensions: ['.js', '.json', '.jsx']
}
```

#### `resolve.extensions`

- Type: `Array<String>`
- Default: `['.js', '.json']`

The file extensions to try when resolving modules.

```javascript
extensions: ['.js', '.json', '.jsx', '.ts', '.tsx']
```

### `module`

- Type: `Object`
- Optional

Configures how different modules are treated.

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

#### `module.rules`

- Type: `Array<Object>`
- Default: `[]`

An array of rules for how modules are processed.

```javascript
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
```

##### `module.rules[].test`

- Type: `String`
- Required: `true`

A regex pattern that matches the files to be processed by this rule.

```javascript
test: '\\.js$'
```

##### `module.rules[].use`

- Type: `Array<String>`
- Required: `true`

An array of loader paths to be applied to the matched files. The loaders are applied from right to left.

```javascript
use: [
  path.resolve(__dirname, './loaders/babel-loader.js')
]
```

### `plugins`

- Type: `Array<String>`
- Default: `[]`

An array of plugin names to be applied to the compilation.

```javascript
plugins: [
  'EmitPlugin',
  'HtmlWebpackPlugin'
]
```

## Next Steps

- [Plugins](/guide/plugins): Learn about the available plugins
- [Loaders](/guide/loaders): Learn about the available loaders
- [Architecture](/guide/architecture): Understand the architecture of mini-rspack
