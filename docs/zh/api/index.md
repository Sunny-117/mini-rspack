# API 概述

mini-rspack 提供了类似 webpack 的 JavaScript API。本节记录了可用的 API。

## webpack

创建编译器实例的主函数。

```javascript
const { webpack } = require('mini-rspack');
const compiler = webpack(options);
```

### 参数

- `options` (Object): webpack 配置选项

### 返回值

- `Compiler`: 编译器实例

## 编译器 (Compiler)

`Compiler` 是 mini-rspack 的主入口点。它管理编译过程并为插件提供钩子。

```javascript
const compiler = webpack(options);

// 运行编译器
compiler.run((err, stats) => {
  // ...
});

// 监听模式
compiler.watch((err, stats) => {
  // ...
});
```

### 方法

#### run(callback)

运行编译器一次。

- `callback` (Function): 编译完成时调用的回调函数
  - `err` (Error): 如果编译失败，则为错误对象
  - `stats` (Stats): 包含有关编译的信息的统计对象

#### watch(callback)

以监听模式运行编译器，当文件变更时自动重新构建。

- `callback` (Function): 每次编译完成时调用的回调函数
  - `err` (Error): 如果编译失败，则为错误对象
  - `stats` (Stats): 包含有关编译的信息的统计对象

### 属性

#### hooks

`hooks` 属性提供对编译器钩子的访问。

```javascript
compiler.hooks.done.tap('MyPlugin', (stats) => {
  console.log('编译完成!');
});
```

可用的钩子：

- `run`: 编译开始前调用
- `emit`: 将资源输出到输出目录前调用
- `done`: 编译完成时调用

## 统计信息 (Stats)

`Stats` 对象包含有关编译的信息。

```javascript
compiler.run((err, stats) => {
  console.log(stats.entries); // 入口点数组
  console.log(stats.chunks); // 块数组
  console.log(stats.modules); // 模块数组
  console.log(stats.files); // 输出文件数组
  console.log(stats.assets); // 资源数组
});
```

### 属性

- `entries` (Array): 应用程序的入口点
- `chunks` (Array): 编译期间生成的块
- `modules` (Array): 编译期间处理的模块
- `files` (Array): 编译期间生成的输出文件
- `assets` (Array): 编译期间生成的资源
