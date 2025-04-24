# 配置

mini-rspack 提供了与 webpack 兼容的配置 API。本页面记录了可用的配置选项。

## 配置对象

配置对象传递给 `createCompiler` 函数：

```javascript
const { createCompiler } = require('mini-rspack');
const compiler = createCompiler({
  // 配置选项
});
```

## 基本配置

以下是一个基本配置示例：

```javascript
const path = require('path');
const { createCompiler } = require('mini-rspack');

const compiler = createCompiler({
  mode: 'development',
  devtool: false,
  watch: false,
  context: './',
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
          path.resolve(__dirname, './loaders/babel-loader.js')
        ]
      }
    ]
  },
  plugins: [
    'EmitPlugin'
  ]
});
```

## 配置选项

### `mode`

- 类型：`String`
- 默认值：`'production'`
- 可能的值：`'development'`, `'production'`

设置编译模式。这会影响代码的打包和优化方式。

```javascript
mode: 'development'
```

### `devtool`

- 类型：`Boolean`
- 默认值：`false`

控制源映射生成。

```javascript
devtool: false
```

### `watch`

- 类型：`Boolean`
- 默认值：`false`

启用监视模式，当文件更改时自动重新构建。

```javascript
watch: true
```

### `context`

- 类型：`String`
- 默认值：`'./'`

解析入口点和加载器的基本目录。

```javascript
context: path.resolve(__dirname, 'src')
```

### `entry`

- 类型：`Object`
- 必需：`true`

定义应用程序的入口点。

```javascript
entry: {
  entries: {
    main: './src/index.js',
    vendor: './src/vendor.js'
  }
}
```

### `output`

- 类型：`Object`
- 必需：`true`

配置如何以及在何处输出打包文件。

```javascript
output: {
  path: path.resolve(__dirname, 'dist'),
  filename: '[name].js'
}
```

#### `output.path`

- 类型：`String`
- 必需：`true`

打包文件的输出目录。

```javascript
path: path.resolve(__dirname, 'dist')
```

#### `output.filename`

- 类型：`String`
- 默认值：`'[name].js'`

打包文件的文件名模式。`[name]` 占位符将替换为入口点名称。

```javascript
filename: '[name].[contenthash].js'
```

### `resolve`

- 类型：`Object`
- 可选

配置模块如何解析。

```javascript
resolve: {
  extensions: ['.js', '.json', '.jsx']
}
```

#### `resolve.extensions`

- 类型：`Array<String>`
- 默认值：`['.js', '.json']`

解析模块时尝试的文件扩展名。

```javascript
extensions: ['.js', '.json', '.jsx', '.ts', '.tsx']
```

### `module`

- 类型：`Object`
- 可选

配置不同模块的处理方式。

```javascript
module: {
  rules: [
    {
      test: '\\.js$',
      use: [
        path.resolve(__dirname, './loaders/babel-loader.js')
      ]
    }
  ]
}
```

#### `module.rules`

- 类型：`Array<Object>`
- 默认值：`[]`

模块处理规则数组。

```javascript
rules: [
  {
    test: '\\.js$',
    use: [
      path.resolve(__dirname, './loaders/babel-loader.js')
    ]
  },
  {
    test: '\\.css$',
    use: [
      path.resolve(__dirname, './loaders/style-loader.js'),
      path.resolve(__dirname, './loaders/css-loader.js')
    ]
  }
]
```

##### `module.rules[].test`

- 类型：`String`
- 必需：`true`

匹配要处理的文件的正则表达式模式。

```javascript
test: '\\.js$'
```

##### `module.rules[].use`

- 类型：`Array<String>`
- 必需：`true`

应用于匹配文件的加载器路径数组。加载器从右到左应用。

```javascript
use: [
  path.resolve(__dirname, './loaders/babel-loader.js')
]
```

### `plugins`

- 类型：`Array<String>`
- 默认值：`[]`

应用于编译的插件名称数组。

```javascript
plugins: [
  'EmitPlugin',
  'HtmlWebpackPlugin'
]
```

## 下一步

- [插件](/zh/guide/plugins)：了解可用的插件
- [加载器](/zh/guide/loaders)：了解可用的加载器
- [架构](/zh/guide/architecture)：了解 mini-rspack 的架构
