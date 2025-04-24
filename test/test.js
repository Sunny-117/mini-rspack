const path = require('path');
const { rspack } = require('../index');
const EmitPlugin = require('../plugins/emit-plugin');

// Create rspack options similar to the JS version
const rspackOptions = {
  mode: 'production',
  devtool: false,
  watch: false,
  context: process.cwd(),
  entry: {
    entries: {
      entry1: './test/src/entry1.js',
      entry2: './test/src/entry2.js',
      'entry-complex': './test/src/entry-complex.js',
      'simple-test': './test/src/simple-test.js',
      'esm-entry': './test/src/esm-entry.js',
      // 'code-splitting': './test/src/code-splitting.js',
      // 'app': './test/src/app.js'
    }
  },
  output: {
    path: path.join(__dirname, '../dist'),
    filename: '[name].js'
  },
  resolve: {
    extensions: ['.js', '.jsx', '.json']
  },
  module: {
    rules: [
      {
        test: '.js',
        use: [
          './loaders/logger-loader.js',
          './loaders/babel-loader.js',
          './loaders/advanced-loader.js'
        ]
      },
      {
        test: '.css',
        use: [
          './loaders/css-loader.js'
        ]
      },
      {
        test: '.svg',
        use: [
          './loaders/file-loader.js'
        ]
      }
    ]
  },
  plugins: [
    // 使用插件名称，Rust 会自动查找对应的插件文件
    'EmitPlugin',
    'HtmlWebpackPlugin',
    'MiniCssExtractPlugin',
    'BannerPlugin'
    // 'AdvancedPlugin' - 暂时移除，因为这个插件可能不存在
  ]
};

// Create a compiler instance
const compiler = rspack(rspackOptions);

// 调用run方法，可以启动编译
compiler.run((err, stats) => {
  if (err) {
    console.error('Compilation failed:', err);
    return;
  }

  console.log('Compilation successful!');
  console.log('Stats:', JSON.stringify(stats, null, 2));
});

// 也可以使用watch方法进行监听模式编译
// compiler.watch((err, stats) => {
//   if (err) {
//     console.error('Watch compilation failed:', err);
//     return;
//   }
//   console.log('Watch compilation successful!');
// });
