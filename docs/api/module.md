# Module API

A `Module` represents a module in the dependency graph. It contains information about the module's source code, dependencies, and how it should be processed.

## Properties

### id

The `id` property is a string that uniquely identifies the module.

```javascript
console.log(module.id);
```

#### Implementation Details

```rust
#[napi(object)]
pub struct Module {
    pub id: String,
    // ...
}
```

### name

The `name` property is a string that represents the name of the module.

```javascript
console.log(module.name);
```

#### Implementation Details

```rust
#[napi(object)]
pub struct Module {
    // ...
    pub name: String,
    // ...
}
```

### source

The `source` property is a string that contains the source code of the module.

```javascript
console.log(module.source);
```

#### Implementation Details

```rust
#[napi(object)]
pub struct Module {
    // ...
    pub source: String,
    // ...
}
```

### dependencies

The `dependencies` property is an array of dependencies of the module.

```javascript
console.log(module.dependencies);
```

#### Implementation Details

```rust
#[napi(object)]
pub struct Module {
    // ...
    pub dependencies: Vec<Dependency>,
}

#[napi(object)]
pub struct Dependency {
    pub dep_module_id: String,
    pub dep_module_path: String,
}
```

## Methods

### new(id, name)

Creates a new module with the given id and name.

```javascript
const module = Module.new('./src/index.js', 'index');
```

#### Parameters

- `id` (String): The id of the module
- `name` (String): The name of the module

#### Returns

- `Module`: A new module instance

#### Implementation Details

```rust
impl Module {
    pub fn new(id: String, name: String) -> Self {
        Self {
            id,
            name,
            source: String::new(),
            dependencies: Vec::new(),
        }
    }
}
```

### addDependency(dependency)

Adds a dependency to the module.

```javascript
module.addDependency({
    dep_module_id: './dep.js',
    dep_module_path: './src/dep.js',
});
```

#### Parameters

- `dependency` (Dependency): The dependency to add

#### Implementation Details

```rust
impl Module {
    pub fn add_dependency(&mut self, dependency: Dependency) {
        self.dependencies.push(dependency);
    }
}
```

### setSource(source)

Sets the source code of the module.

```javascript
module.setSource('console.log("Hello, World!");');
```

#### Parameters

- `source` (String): The source code of the module

#### Implementation Details

```rust
impl Module {
    pub fn set_source(&mut self, source: String) {
        self.source = source;
    }
}
```

## Dependency API

A `Dependency` represents a dependency of a module.

### Properties

#### dep_module_id

The `dep_module_id` property is a string that uniquely identifies the dependency module.

```javascript
console.log(dependency.dep_module_id);
```

#### Implementation Details

```rust
#[napi(object)]
pub struct Dependency {
    pub dep_module_id: String,
    // ...
}
```

#### dep_module_path

The `dep_module_path` property is a string that represents the path of the dependency module.

```javascript
console.log(dependency.dep_module_path);
```

#### Implementation Details

```rust
#[napi(object)]
pub struct Dependency {
    // ...
    pub dep_module_path: String,
}
```

## Next Steps

- [Compiler](/api/compiler): Learn about the Compiler API
- [Compilation](/api/compilation): Learn about the Compilation API
- [Hooks](/api/hooks): Learn about the Hook API
