# 加载器

加载器是应用于模块源代码的转换。它们允许您在导入或"加载"文件时预处理文件。本页面记录了可用的加载器以及如何创建自己的加载器。

## 内置加载器

mini-rspack 附带了一些内置加载器：

### file-loader

`file-loader` 将文件发送到输出目录并返回公共 URL。

```javascript
module: {
  rules: [
    {
      test: '\\.png$',
      use: [
        path.resolve(__dirname, './loaders/file-loader.js')
      ]
    }
  ]
}
```

## 示例加载器

### babel-loader

`babel-loader` 将 ES6+ 代码转换为 ES5。

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

### css-loader

`css-loader` 解释 `@import` 和 `url()` 就像 `import/require()` 一样，并将解析它们。

```javascript
module: {
  rules: [
    {
      test: '\\.css$',
      use: [
        path.resolve(__dirname, './loaders/css-loader.js')
      ]
    }
  ]
}
```

### json-loader

`json-loader` 加载 JSON 文件。

```javascript
module: {
  rules: [
    {
      test: '\\.json$',
      use: [
        path.resolve(__dirname, './loaders/json-loader.js')
      ]
    }
  ]
}
```

### html-loader

`html-loader` 将 HTML 导出为字符串。

```javascript
module: {
  rules: [
    {
      test: '\\.html$',
      use: [
        path.resolve(__dirname, './loaders/html-loader.js')
      ]
    }
  ]
}
```

## 创建自定义加载器

您可以通过创建一个 JavaScript 函数来创建自己的加载器，该函数接受源代码作为输入并返回转换后的源代码。

### 加载器结构

```javascript
module.exports = function(source, name, modulePath) {
  // 转换源代码
  const transformedSource = source.replace('Hello', 'Hello from Loader');
  
  return transformedSource;
};
```

### 加载器参数

- `source`：文件内容
- `name`：模块名称
- `modulePath`：模块路径

### 示例：babel-loader 实现

以下是 `babel-loader` 的实现示例：

```javascript
/**
 * 一个简单的类似 babel 的加载器，将 ES6+ 代码转换为 ES5
 */
module.exports = function(source) {
  // 这是一个简化版本，只做一些基本转换
  // 在真正的 babel-loader 中，它会使用 babel 核心来转换代码
  
  // 将箭头函数转换为常规函数
  let transformed = source.replace(/const\s+(\w+)\s*=\s*\(([^)]*)\)\s*=>\s*{([^}]*)}/g, 'const $1 = function($2) {$3}');
  
  // 将 let/const 转换为 var
  transformed = transformed.replace(/let\s+/g, 'var ');
  transformed = transformed.replace(/const\s+/g, 'var ');
  
  // 将模板字面量转换为字符串连接
  transformed = transformed.replace(/`([^`]*)\${([^}]*)}`/g, '"$1" + $2');
  
  // 添加注释以指示代码已转换
  return `// Transformed by babel-loader\n${transformed}`;
};
```

### 示例：css-loader 实现

以下是 `css-loader` 的实现示例：

```javascript
/**
 * 一个简单的 CSS 加载器，将 CSS 转换为 JavaScript 模块
 */
module.exports = function(source, name, modulePath) {
  console.log(`CSS loader processing file: ${modulePath}`);
  
  // 转义引号和换行符
  const escapedCSS = source
    .replace(/\\/g, '\\\\')
    .replace(/\'/g, '\\\'')
    .replace(/\n/g, '\\n');
  
  // 将 CSS 转换为导出 CSS 字符串的 JavaScript 模块
  const jsModule = `
    // CSS Module
    const css = '${escapedCSS}';
    
    // 创建样式元素
    const styleEl = document.createElement('style');
    styleEl.textContent = css;
    
    // 在导入模块时附加到头部
    document.head.appendChild(styleEl);
    
    // 导出 CSS 字符串
    module.exports = css;
  `;
  
  console.log('CSS transformation complete');
  return jsModule;
};
```

### 示例：json-loader 实现

以下是 `json-loader` 的实现示例：

```javascript
/**
 * 一个简单的 JSON 加载器，将 JSON 转换为 JavaScript 模块
 */
module.exports = function(source, name, modulePath) {
  console.log(`JSON loader processing file: ${modulePath}`);
  
  try {
    // 解析 JSON 以验证它
    const json = JSON.parse(source);
    
    // 将 JSON 转换为 JavaScript 模块
    const jsModule = `module.exports = ${source};`;
    
    console.log('JSON transformation complete');
    return jsModule;
  } catch (error) {
    console.error(`Error parsing JSON in ${modulePath}: ${error.message}`);
    throw new Error(`Invalid JSON in ${modulePath}: ${error.message}`);
  }
};
```

### 示例：html-loader 实现

以下是 `html-loader` 的实现示例：

```javascript
/**
 * 一个简单的 HTML 加载器，将 HTML 转换为 JavaScript 模块
 */
module.exports = function(source, name, modulePath) {
  console.log(`HTML loader processing file: ${modulePath}`);
  
  // 转义引号和换行符
  const escapedHTML = source
    .replace(/\\/g, '\\\\')
    .replace(/\'/g, '\\\'')
    .replace(/\n/g, '\\n');
  
  // 将 HTML 转换为 JavaScript 模块
  const jsModule = `
    // HTML Module
    const html = '${escapedHTML}';
    
    // 导出 HTML 字符串
    module.exports = html;
    
    // 导出一个创建 DOM 元素的函数
    module.exports.createDOM = function() {
      const template = document.createElement('template');
      template.innerHTML = html.trim();
      return template.content.firstChild;
    };
  `;
  
  console.log('HTML transformation complete');
  return jsModule;
};
```

## 链式加载器

您可以链接多个加载器来转换文件。加载器从右到左应用。

```javascript
module: {
  rules: [
    {
      test: '\\.css$',
      use: [
        path.resolve(__dirname, './loaders/style-loader.js'),
        path.resolve(__dirname, './loaders/css-loader.js')
      ]
    }
  ]
}
```

在此示例中，文件首先由 `css-loader` 处理，然后结果由 `style-loader` 处理。

## 使用加载器

要使用加载器，请将其添加到配置中的 `module.rules` 数组中：

```javascript
const { createCompiler } = require('mini-rspack');
const compiler = createCompiler({
  // ...
  module: {
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
  }
});
```

## 下一步

- [插件](/zh/guide/plugins)：了解可用的插件
- [配置](/zh/guide/configuration)：了解所有配置选项
- [架构](/zh/guide/architecture)：了解 mini-rspack 的架构
