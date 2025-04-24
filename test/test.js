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
      entry1: './src/entry1.js',
      entry2: './src/entry2.js',
      'entry-complex': './src/entry-complex.js',
      'simple-test': './src/simple-test.js'
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
        test: '\\.js$',
        use: [
          path.resolve(__dirname, '../loaders/logger-loader.js')
        ]
      }
    ]
  },
  plugins: [
    'EmitPlugin' // Changed to string to match our simplified Rust implementation
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
