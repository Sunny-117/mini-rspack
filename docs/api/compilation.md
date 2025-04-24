# Compilation API

A `Compilation` represents a single build of the application. It contains the modules, chunks, and assets generated during the build.

## Properties

### modules

The `modules` property is an array of modules processed during the compilation.

```javascript
console.log(compilation.modules);
```

#### Implementation Details

```rust
pub struct Compilation {
    pub modules: Vec<Module>,
    // ...
}
```

### chunks

The `chunks` property is an array of chunks generated during the compilation.

```javascript
console.log(compilation.chunks);
```

#### Implementation Details

```rust
pub struct Compilation {
    // ...
    pub chunks: Vec<Chunk>,
    // ...
}
```

### assets

The `assets` property is a map of asset names to asset content.

```javascript
console.log(compilation.assets);
```

#### Implementation Details

```rust
pub struct Compilation {
    // ...
    pub assets: HashMap<String, String>,
    // ...
}
```

### files

The `files` property is an array of output file names.

```javascript
console.log(compilation.files);
```

#### Implementation Details

```rust
pub struct Compilation {
    // ...
    pub files: Vec<String>,
    // ...
}
```

### entries

The `entries` property is a map of entry point names to entry point paths.

```javascript
console.log(compilation.entries);
```

#### Implementation Details

```rust
pub struct Compilation {
    // ...
    pub entries: HashMap<String, String>,
    // ...
}
```

### options

The `options` property provides access to the compiler options.

```javascript
console.log(compilation.options);
```

#### Implementation Details

```rust
pub struct Compilation {
    // ...
    pub options: RspackOptions,
    // ...
}
```

### hooks

The `hooks` property provides access to the compilation hooks.

```javascript
compilation.hooks.emit.tap('MyPlugin', (assets) => {
  console.log('Emitting assets!');
});
```

#### Available Hooks

- `emit`: Called before emitting assets to the output directory

#### Implementation Details

```rust
pub struct CompilationHooks {
    pub emit: SyncHook<Option<&mut HashMap<String, String>>>,
}

impl CompilationHooks {
    pub fn new() -> Self {
        Self {
            emit: SyncHook::new("emit"),
        }
    }
}

pub struct Compilation {
    // ...
    pub hooks: CompilationHooks,
}
```

## Methods

### addModule(module)

Adds a module to the compilation.

```javascript
compilation.addModule(module);
```

#### Parameters

- `module` (Module): The module to add

#### Implementation Details

```rust
impl Compilation {
    pub fn add_module(&mut self, module: Module) {
        self.modules.push(module);
    }
}
```

### addChunk(chunk)

Adds a chunk to the compilation.

```javascript
compilation.addChunk(chunk);
```

#### Parameters

- `chunk` (Chunk): The chunk to add

#### Implementation Details

```rust
impl Compilation {
    pub fn add_chunk(&mut self, chunk: Chunk) {
        self.chunks.push(chunk);
    }
}
```

### addAsset(name, content)

Adds an asset to the compilation.

```javascript
compilation.addAsset('main.js', 'console.log("Hello, World!");');
```

#### Parameters

- `name` (String): The name of the asset
- `content` (String): The content of the asset

#### Implementation Details

```rust
impl Compilation {
    pub fn add_asset(&mut self, name: String, content: String) {
        self.assets.insert(name.clone(), content);
        self.files.push(name);
    }
}
```

### addEntry(name, path)

Adds an entry point to the compilation.

```javascript
compilation.addEntry('main', './src/index.js');
```

#### Parameters

- `name` (String): The name of the entry point
- `path` (String): The path of the entry point

#### Implementation Details

```rust
impl Compilation {
    pub fn add_entry(&mut self, name: String, path: String) {
        self.entries.insert(name, path);
    }
}
```

## Next Steps

- [Compiler](/api/compiler): Learn about the Compiler API
- [Module](/api/module): Learn about the Module API
- [Hooks](/api/hooks): Learn about the Hook API
