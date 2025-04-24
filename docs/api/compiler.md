# Compiler API

The `Compiler` is the main entry point of mini-rspack. It manages the compilation process and provides hooks for plugins to tap into.

## Creating a Compiler

```javascript
const { createCompiler } = require('mini-rspack');
const compiler = createCompiler(options);
```

## Methods

### run(callback)

Runs the compiler once.

```javascript
compiler.run((err, stats) => {
  if (err) {
    console.error('Compilation failed:', err);
    return;
  }
  
  console.log('Compilation successful!');
  console.log('Stats:', stats);
});
```

#### Parameters

- `callback` (Function): A callback function that is called when the compilation is complete
  - `err` (Error): An error object if the compilation failed
  - `stats` (Stats): A stats object containing information about the compilation

#### Implementation Details

```rust
#[napi]
pub fn run(&mut self, callback: JsFunction) -> Result<()> {
    // Create a threadsafe function from the callback
    let tsfn = callback.create_threadsafe_function(0, |ctx| {
        let mut data = ctx.value.unwrap();
        let err = ctx.env.get_undefined().unwrap();
        let stats = ctx.env.to_js_value(&data).unwrap();
        Ok(vec![err, stats])
    })?;

    // Run the compilation process
    let result = run_compiler(self);
    match result {
        Ok(stats) => {
            // Call the callback with the stats
            tsfn.call(Ok(stats), ThreadsafeFunctionCallMode::Blocking);
        }
        Err(err) => {
            // Call the callback with the error
            let error = ctx.env.create_error(err.to_string()).unwrap();
            tsfn.call(Err(error), ThreadsafeFunctionCallMode::Blocking);
        }
    }

    Ok(())
}
```

### watch(callback)

Runs the compiler in watch mode, which automatically rebuilds when files change.

```javascript
compiler.watch((err, stats) => {
  if (err) {
    console.error('Watch compilation failed:', err);
    return;
  }
  
  console.log('Watch compilation successful!');
  console.log('Stats:', stats);
});
```

#### Parameters

- `callback` (Function): A callback function that is called when each compilation is complete
  - `err` (Error): An error object if the compilation failed
  - `stats` (Stats): A stats object containing information about the compilation

#### Implementation Details

```rust
#[napi]
pub fn watch(&mut self, callback: JsFunction) -> Result<()> {
    // Create a threadsafe function from the callback
    let tsfn = callback.create_threadsafe_function(0, |ctx| {
        let mut data = ctx.value.unwrap();
        let err = ctx.env.get_undefined().unwrap();
        let stats = ctx.env.to_js_value(&data).unwrap();
        Ok(vec![err, stats])
    })?;

    // Start the watch process
    watch_compiler(self, tsfn);

    Ok(())
}
```

## Properties

### hooks

The `hooks` property provides access to the compiler hooks.

```javascript
compiler.hooks.done.tap('MyPlugin', (stats) => {
  console.log('Compilation done!');
});
```

#### Available Hooks

- `run`: Called before the compilation starts
- `emit`: Called before emitting assets to the output directory
- `done`: Called when the compilation is complete

#### Implementation Details

```rust
pub struct CompilerHooks {
    pub run: SyncHook<Option<&mut HashMap<String, String>>>,
    pub emit: SyncHook<Option<&mut HashMap<String, String>>>,
    pub done: SyncHook<Option<&mut HashMap<String, String>>>,
}

impl CompilerHooks {
    pub fn new() -> Self {
        Self {
            run: SyncHook::new("run"),
            emit: SyncHook::new("emit"),
            done: SyncHook::new("done"),
        }
    }
}
```

### options

The `options` property provides access to the compiler options.

```javascript
console.log(compiler.options.entry);
console.log(compiler.options.output);
```

#### Implementation Details

```rust
#[napi(object)]
pub struct Compiler {
    pub options: RspackOptions,
    pub hooks: CompilerHooks,
}
```

## Next Steps

- [Compilation](/api/compilation): Learn about the Compilation API
- [Module](/api/module): Learn about the Module API
- [Hooks](/api/hooks): Learn about the Hook API
