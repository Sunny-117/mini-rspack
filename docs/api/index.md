# API Overview

mini-rspack provides a JavaScript API similar to webpack. This section documents the available APIs.

## rspack

The main function to create a compiler instance.

```javascript
const { rspack } = require('mini-rspack');
const compiler = rspack(options);
```

### Parameters

- `options` (Object): The rspack configuration options

### Returns

- `Compiler`: A compiler instance

## Compiler

The `Compiler` is the main entry point of mini-rspack. It manages the compilation process and provides hooks for plugins to tap into.

```javascript
const compiler = rspack(options);

// Run the compiler
compiler.run((err, stats) => {
  // ...
});

// Watch mode
compiler.watch((err, stats) => {
  // ...
});
```

### Methods

#### run(callback)

Runs the compiler once.

- `callback` (Function): A callback function that is called when the compilation is complete
  - `err` (Error): An error object if the compilation failed
  - `stats` (Stats): A stats object containing information about the compilation

#### watch(callback)

Runs the compiler in watch mode, which automatically rebuilds when files change.

- `callback` (Function): A callback function that is called when each compilation is complete
  - `err` (Error): An error object if the compilation failed
  - `stats` (Stats): A stats object containing information about the compilation

### Properties

#### hooks

The `hooks` property provides access to the compiler hooks.

```javascript
compiler.hooks.done.tap('MyPlugin', (stats) => {
  console.log('Compilation done!');
});
```

Available hooks:

- `run`: Called before the compilation starts
- `emit`: Called before emitting assets to the output directory
- `done`: Called when the compilation is complete

## Stats

The `Stats` object contains information about the compilation.

```javascript
compiler.run((err, stats) => {
  console.log(stats.entries); // Array of entry points
  console.log(stats.chunks); // Array of chunks
  console.log(stats.modules); // Array of modules
  console.log(stats.files); // Array of output files
  console.log(stats.assets); // Array of assets
});
```

### Properties

- `entries` (Array): The entry points of the application
- `chunks` (Array): The chunks generated during compilation
- `modules` (Array): The modules processed during compilation
- `files` (Array): The output files generated during compilation
- `assets` (Array): The assets generated during compilation
