# Hooks API

Hooks are used to tap into different stages of the compilation process. They allow plugins to modify the behavior of the compiler and compilation.

## SyncHook

A `SyncHook` is a synchronous hook that can be tapped into by plugins.

```javascript
const hook = new SyncHook('test');
hook.tap('MyPlugin');
hook.call();
```

### Properties

#### name

The `name` property is a string that represents the name of the hook.

```javascript
console.log(hook.name);
```

#### Implementation Details

```rust
#[napi(object)]
pub struct SyncHook {
    pub name: String,
    // ...
}
```

#### taps

The `taps` property is an array of strings that represents the names of the plugins that have tapped into the hook.

```javascript
console.log(hook.taps);
```

#### Implementation Details

```rust
#[napi(object)]
pub struct SyncHook {
    // ...
    pub taps: Vec<String>,
}
```

### Methods

#### new(name)

Creates a new hook with the given name.

```javascript
const hook = new SyncHook('test');
```

#### Parameters

- `name` (String): The name of the hook

#### Returns

- `SyncHook`: A new hook instance

#### Implementation Details

```rust
impl SyncHook {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            taps: Vec::new(),
        }
    }
}
```

#### tap(name)

Taps into the hook with the given name.

```javascript
hook.tap('MyPlugin');
```

#### Parameters

- `name` (String): The name of the plugin

#### Implementation Details

```rust
impl SyncHook {
    pub fn tap(&mut self, name: &str) {
        self.taps.push(name.to_string());
    }
}
```

#### call(args)

Calls the hook with the given arguments.

```javascript
hook.call();
```

#### Parameters

- `args` (Any): The arguments to pass to the hook

#### Implementation Details

```rust
impl SyncHook {
    pub fn call(&self, args: Option<&mut HashMap<String, String>>) {
        // Call the tapped functions
        println!("Hook '{}' called with {} taps", self.name, self.taps.len());
        
        // For now, we'll just print the taps
        for tap in &self.taps {
            println!("  - Tap: {}", tap);
        }
    }
}
```

## CompilerHooks

The `CompilerHooks` object contains hooks that are called during the compilation process.

```javascript
compiler.hooks.done.tap('MyPlugin', (stats) => {
  console.log('Compilation done!');
});
```

### Properties

#### run

The `run` hook is called before the compilation starts.

```javascript
compiler.hooks.run.tap('MyPlugin', () => {
  console.log('Compilation starting!');
});
```

#### Implementation Details

```rust
pub struct CompilerHooks {
    pub run: SyncHook<Option<&mut HashMap<String, String>>>,
    // ...
}
```

#### emit

The `emit` hook is called before emitting assets to the output directory.

```javascript
compiler.hooks.emit.tap('MyPlugin', (compilation) => {
  console.log('Emitting assets!');
});
```

#### Implementation Details

```rust
pub struct CompilerHooks {
    // ...
    pub emit: SyncHook<Option<&mut HashMap<String, String>>>,
    // ...
}
```

#### done

The `done` hook is called when the compilation is complete.

```javascript
compiler.hooks.done.tap('MyPlugin', (stats) => {
  console.log('Compilation done!');
});
```

#### Implementation Details

```rust
pub struct CompilerHooks {
    // ...
    pub done: SyncHook<Option<&mut HashMap<String, String>>>,
}
```

## CompilationHooks

The `CompilationHooks` object contains hooks that are called during the compilation process.

```javascript
compilation.hooks.emit.tap('MyPlugin', (assets) => {
  console.log('Emitting assets!');
});
```

### Properties

#### emit

The `emit` hook is called before emitting assets to the output directory.

```javascript
compilation.hooks.emit.tap('MyPlugin', (assets) => {
  console.log('Emitting assets!');
});
```

#### Implementation Details

```rust
pub struct CompilationHooks {
    pub emit: SyncHook<Option<&mut HashMap<String, String>>>,
}
```

## Next Steps

- [Compiler](/api/compiler): Learn about the Compiler API
- [Compilation](/api/compilation): Learn about the Compilation API
- [Module](/api/module): Learn about the Module API
