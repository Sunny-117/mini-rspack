# 快速开始

本指南将帮助您开始使用 mini-rspack。

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

## 基本用法

创建一个使用 mini-rspack 的 JavaScript 文件：

```javascript
const path = require('path');
const { rspack } = require('mini-rspack');

// 创建 rspack 选项
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
  }
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
```

## 监听模式

mini-rspack 支持监听模式，当文件变更时自动重新构建：

```javascript
// 在选项中启用监听模式
const rspackOptions = {
  // ...
  watch: true,
  // ...
};

// 创建编译器实例
const compiler = rspack(rspackOptions);

// 以监听模式运行
compiler.watch((err, stats) => {
  if (err) {
    console.error('监听编译失败:', err);
    return;
  }
  console.log('监听编译成功!');
});
```

## 下一步

- [配置](/zh/guide/configuration)：了解所有配置选项
- [插件](/zh/guide/plugins)：使用插件扩展 mini-rspack
- [加载器](/zh/guide/loaders)：使用加载器转换模块内容
