# 架构

mini-rspack 遵循类似于 webpack 的模块化架构，组件设计为协同工作，将源代码转换为打包输出。本页面解释了高级架构以及不同组件如何交互。

## 概述

mini-rspack 的架构由以下主要组件组成：

1. **Compiler**：管理编译过程的主入口点
2. **Compilation**：表示应用程序的单次构建
3. **Module**：表示依赖图中的一个模块
4. **Loader**：转换模块内容
5. **Plugin**：扩展打包工具的功能
6. **Hook System**：为插件提供挂钩到编译不同阶段的方式
7. **Resolver**：解析模块路径并处理模块解析算法

## 组件交互

以下是这些组件在打包过程中如何交互：

1. 使用配置对象创建 **Compiler**
2. 当调用 `compiler.run()` 时，它创建一个新的 **Compilation** 实例
3. **Compilation** 读取入口点并开始构建依赖图
4. 对于每个模块，它：
   - 使用 **Resolver** 解析模块路径
   - 读取模块内容
   - 应用 **Loaders** 转换内容
   - 解析转换后的内容以提取依赖项
   - 将模块添加到依赖图
5. 一旦处理完所有模块，**Compilation** 创建块并生成资源
6. 在此过程中，调用 **Hooks**，允许 **Plugins** 修改行为

## Rust 和 JavaScript 交互

mini-rspack 使用 Rust 实现核心功能并提供 JavaScript API。以下是两种语言如何交互：

1. JavaScript API 在 `index.js` 中定义，提供类似 webpack 的接口
2. Rust 代码使用 napi-rs 编译为原生 Node.js 模块
3. JavaScript API 调用 Rust 代码执行实际打包
4. 回调和钩子允许 JavaScript 代码（如插件和加载器）与 Rust 核心交互

## 代码结构

代码库组织如下：

```
mini-rspack/
├── src/                 # Rust 源代码
│   ├── lib.rs           # 主库入口点
│   ├── compiler.rs      # 编译器实现
│   ├── compilation.rs   # 编译实现
│   ├── module.rs        # 模块实现
│   ├── loader.rs        # 加载器系统
│   ├── loader_runner.rs # 加载器运行器实现
│   ├── plugin.rs        # 插件系统
│   ├── hook.rs          # 钩子系统
│   └── utils.rs         # 实用函数
├── index.js             # JavaScript API
├── loaders/             # 示例加载器
├── plugins/             # 示例插件
└── tests/               # 测试文件
    ├── js/              # JavaScript 测试
    └── rust/            # Rust 测试
```

## 编译过程

mini-rspack 中的编译过程遵循以下步骤：

1. **初始化**：使用提供的配置创建编译器实例
2. **入口解析**：解析配置中指定的入口点
3. **模块处理**：对于每个模块：
   - 解析模块路径
   - 读取模块内容
   - 应用加载器转换内容
   - 解析转换后的内容以提取依赖项
   - 将模块添加到依赖图
4. **依赖解析**：解析每个模块的依赖项并重复模块处理步骤
5. **块创建**：根据入口点和动态导入将模块分组为块
6. **资源生成**：从块生成资源（JavaScript 文件）
7. **插件处理**：允许插件修改生成的资源
8. **输出**：将资源写入输出目录

## 技术实现细节

### 模块解析

mini-rspack 使用正则表达式模式从 JavaScript 模块中提取依赖项：

```rust
// 提取 CommonJS requires
let re_require = Regex::new(r#"require\(['"](.*?)['"]"#).unwrap();
for cap in re_require.captures_iter(source) {
    let dep = cap.get(1).unwrap().as_str();
    dependencies.push(dep.to_string());
}

// 提取 ES 模块导入
let re_import = Regex::new(r#"import\s+.*?from\s+['"]([^'"]+)['"]"#).unwrap();
for cap in re_import.captures_iter(source) {
    let dep = cap.get(1).unwrap().as_str();
    dependencies.push(dep.to_string());
}

// 提取动态导入
let re_dynamic_import = Regex::new(r#"import\(['"]([^'"]+)['"]\)"#).unwrap();
for cap in re_dynamic_import.captures_iter(source) {
    let dep = cap.get(1).unwrap().as_str();
    dependencies.push(dep.to_string());
}
```

### 加载器运行器

加载器是转换模块内容的 JavaScript 函数。加载器运行器按顺序执行这些函数：

```rust
pub fn apply_loaders(source: &str, loaders: &Vec<Loader>, module_path: &Path) -> Result<String> {
    // 如果没有加载器，按原样返回源
    if loaders.is_empty() {
        return Ok(source.to_string());
    }

    // 为加载器运行器创建临时文件
    let mut temp_file = tempfile::NamedTempFile::new()?;
    let loader_runner_path = temp_file.path();

    // 生成加载器运行器代码
    let loader_runner_code = generate_loader_runner(source, loaders, module_path)?;

    // 将加载器运行器代码写入临时文件
    std::fs::write(loader_runner_path, loader_runner_code)?;

    // 执行加载器运行器
    let output = std::process::Command::new("node")
        .arg(loader_runner_path)
        .output()?;

    // 检查执行是否成功
    if !output.status.success() {
        let error_message = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow::anyhow!("Loader execution failed: {}", error_message));
    }

    // 从输出获取转换后的源
    let transformed_source = String::from_utf8_lossy(&output.stdout).to_string();

    Ok(transformed_source)
}
```

### 钩子系统

钩子系统允许插件挂钩到编译过程的不同阶段：

```rust
pub struct SyncHook {
    pub name: String,
    pub taps: Vec<String>,
}

impl SyncHook {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            taps: Vec::new(),
        }
    }

    pub fn tap(&mut self, name: &str) {
        self.taps.push(name.to_string());
    }

    pub fn call(&self, args: Option<&mut HashMap<String, String>>) {
        // 调用挂钩的函数
        println!("Hook '{}' called with {} taps", self.name, self.taps.len());

        // 现在，我们只打印挂钩
        for tap in &self.taps {
            println!("  - Tap: {}", tap);
        }
    }
}
```

## 下一步

- [快速开始](/zh/guide/getting-started)：了解如何安装和使用 mini-rspack
- [API 概述](/zh/api/)：探索 API 文档
- [技术实现](/zh/api/)：深入了解技术细节
