# mini-rspack

mini-rspack 是一个使用 Rust 和 Node.js 绑定实现的简化版 Rspack 打包工具。它展示了如何使用 Rust 创建高性能的 JavaScript 打包工具。

[English](README.md) | [中文](README_zh.md)

## 简介

mini-rspack 是一个概念验证性的 JavaScript 打包工具，它使用 Rust 实现核心功能并提供类似 webpack 的 JavaScript API。它展示了如何：

- 使用 napi-rs 创建基于 Rust 的 Node.js 原生模块
- 实现简化的打包工具架构
- 解析和分析 JavaScript 模块
- 解析模块依赖关系
- 使用加载器转换代码
- 使用插件扩展功能
- 生成打包后的输出
- 提供熟悉的类 webpack API

## 特性

- 兼容 webpack 的 API，使用 compiler.run() 方法
- 支持 CommonJS 和 ES 模块两种格式
- 入口点处理，支持多入口
- 使用自定义解析器的模块解析
- 插件系统和钩子（类似 Tapable）
- 用于转换模块内容的加载器系统
- 用于开发的监听模式

## 安装

```bash
# 克隆仓库
git clone https://github.com/Sunny-117/mini-rspack.git
cd mini-rspack

# 安装依赖
npm install

# 构建 Rust 代码
npm run build
```

## 使用方法

创建类似 webpack 的配置文件：

```javascript
const path = require('path');
const { rspack } = require('mini-rspack');

const rspackOptions = {
  mode: 'production',
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
          path.resolve(__dirname, './loaders/my-loader.js')
        ]
      }
    ]
  },
  plugins: [
    'MyPlugin'
  ]
};

// 创建编译器实例
const compiler = rspack(rspackOptions);

// 运行编译器
compiler.run((err, stats) => {
  if (err) {
    console.error('编译失败:', err);
    return;
  }

  console.log('编译成功!');
  console.log('统计信息:', JSON.stringify(stats, null, 2));
});

// 或者使用监听模式
compiler.watch((err, stats) => {
  if (err) {
    console.error('监听编译失败:', err);
    return;
  }
  console.log('监听编译成功!');
});
```

## 架构

mini-rspack 包含以下组件：

1. **Compiler**: 主入口点，管理编译过程。使用 Rust 实现，并提供 JavaScript 绑定。
2. **Compilation**: 表示应用程序的单次构建。管理构建过程并保存编译状态。
3. **Module**: 表示依赖图中的一个模块。包含模块的源代码、依赖关系和处理方式的信息。
4. **Loader**: 转换模块内容。实现为可以链式调用的 JavaScript 函数。
5. **Plugin**: 扩展打包工具的功能。使用钩子系统来介入编译过程的不同阶段。
6. **Hook System**: 类似 webpack 的 Tapable，为插件提供了介入编译不同阶段的方式。
7. **Resolver**: 解析模块路径并处理模块解析算法。

### 技术实现

- **Rust 核心**: 核心打包逻辑使用 Rust 实现，以提高性能。
- **napi-rs**: 用于从 Rust 代码创建 Node.js 原生模块。
- **JavaScript API**: 提供类似 webpack 的 API 进行配置和使用。
- **模块解析**: 使用正则表达式和基本 AST 分析来提取依赖关系。
- **依赖解析**: 实现类似 Node.js 模块解析的路径解析。
- **代码生成**: 生成带有适当模块包装和运行时的 JavaScript 打包文件。

## 开发

```bash
# 以调试模式构建
npm run build:debug

# 运行测试
npm test

```

## 文档

详细文档可在 `docs` 目录中找到。您可以使用以下命令构建和查看文档：

```bash
cd docs
pnpm install
pnpm docs:dev
pnpm docs:build
```

## 教学资源

该项目被设计为学习以下内容的教育资源：

- 现代 JavaScript 打包工具架构
- Rust 和 JavaScript 的互操作性
- 模块解析算法
- 插件和加载器系统
- AST 解析和代码转换
- 构建工具中的性能优化

## 许可证

MIT
