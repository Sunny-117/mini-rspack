# 介绍

mini-rspack 是一个概念验证性的 JavaScript 打包工具，它使用 Rust 实现核心功能并提供类似 webpack 的 JavaScript API。它展示了如何使用 Rust 和 Node.js 原生模块创建高性能的 JavaScript 工具。

## 为什么选择 mini-rspack？

现代 JavaScript 打包工具如 webpack 功能强大，但在处理大型应用程序时可能会变慢。通过在 Rust 中实现核心功能，mini-rspack 旨在提供：

1. **更好的性能**：Rust 的速度和内存效率可以显著提高打包时间
2. **熟悉的 API**：兼容 webpack 的 API 使开发者容易采用
3. **学习资源**：简化的实现有助于理解打包工具架构

## 核心概念

mini-rspack 遵循与 webpack 类似的概念：

### 编译器 (Compiler)

`Compiler` 是 mini-rspack 的主入口点。它管理编译过程并为插件提供钩子。

```javascript
const { webpack } = require('mini-rspack');
const compiler = webpack(options);
```

### 编译 (Compilation)

`Compilation` 表示应用程序的单次构建。它包含构建过程中生成的模块、块和资源。

### 模块 (Module)

`Module` 表示依赖图中的一个模块。它包含有关模块源代码、依赖关系以及如何处理它的信息。

### 加载器 (Loader)

加载器转换模块的内容。它们可用于处理非 JavaScript 文件或转换 JavaScript 代码。

### 插件 (Plugin)

插件扩展 mini-rspack 的功能。它们可以利用编译器和编译提供的钩子来修改构建过程。

## 下一步

- [快速开始](/zh/guide/getting-started)：学习如何安装和使用 mini-rspack
- [配置](/zh/guide/configuration)：探索配置选项
- [插件](/zh/guide/plugins)：学习如何使用和创建插件
- [加载器](/zh/guide/loaders)：学习如何使用和创建加载器
