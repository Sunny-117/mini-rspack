# mini-rspack

mini-rspack 是一个使用 Rust 和 Node.js 绑定实现的简化版 webpack 打包工具。它展示了如何使用 Rust 创建高性能的 JavaScript 打包工具。

[English](README.md) | [中文](README_zh.md)

## 简介

mini-rspack 是一个概念验证性的 JavaScript 打包工具，它使用 Rust 实现核心功能并提供类似 webpack 的 JavaScript API。它展示了如何：

- 使用 napi-rs 创建基于 Rust 的 Node.js 原生模块
- 实现简化的打包工具架构
- 提供熟悉的类 webpack API

## 特性

- 兼容 webpack 的 API
- 入口点处理
- 基本模块解析
- 插件系统和钩子
- 加载器支持
- 监听模式

## 安装

```bash
# 克隆仓库
git clone https://github.com/yourusername/mini-rspack.git
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

1. **Compiler**: 主入口点，管理编译过程
2. **Compilation**: 表示应用程序的单次构建
3. **Module**: 表示依赖图中的一个模块
4. **Loader**: 转换模块内容
5. **Plugin**: 扩展打包工具的功能

## 开发

```bash
# 以调试模式构建
npm run build:debug

# 运行测试
npm test
```

## 许可证

MIT
