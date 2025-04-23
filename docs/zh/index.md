---
layout: home
hero:
  name: mini-rspack
  text: 使用 Rust 实现的简化版 webpack 打包工具
  tagline: 基于 Rust 和 Node.js 的高性能 JavaScript 打包工具
  image:
    src: /logo.png
    alt: mini-rspack
  actions:
    - theme: brand
      text: 开始使用
      link: /zh/guide/
    - theme: alt
      text: 在 GitHub 上查看
      link: https://github.com/Sunny-117/mini-rspack

features:
  - icon: 🚀
    title: 高性能
    details: 使用 Rust 构建，相比基于 JavaScript 的打包工具性能更佳
  - icon: 🔌
    title: 兼容 webpack
    details: 为 webpack 用户提供熟悉的 API，使迁移变得简单
  - icon: 🧩
    title: 可扩展
    details: 支持插件和加载器以扩展功能
  - icon: 🔄
    title: 监听模式
    details: 文件变更时自动重新构建
---

## 什么是 mini-rspack？

mini-rspack 是一个使用 Rust 和 Node.js 绑定实现的简化版 webpack 打包工具。它展示了如何使用 Rust 创建高性能的 JavaScript 打包工具。

## 快速开始

```bash
# 安装
npm install mini-rspack

# 创建配置文件
const { rspack } = require('mini-rspack');
const compiler = rspack(options);

# 运行编译器
compiler.run((err, stats) => {
  console.log('编译成功!');
});
```

## 特性

- **兼容 webpack 的 API**：为 webpack 用户提供熟悉的接口
- **入口点处理**：支持多个入口点
- **基本模块解析**：解析模块之间的依赖关系
- **插件系统和钩子**：通过插件扩展功能
- **加载器支持**：使用加载器转换模块内容
- **监听模式**：文件变更时自动重新构建
