const path = require('path');
const { webpack } = require('../index');
const EmitPlugin = require('../plugins/emit-plugin');

// Create webpack options similar to the JS version
const webpackOptions = {
  mode: 'production',
  devtool: false,
  watch: false,
  context: process.cwd(),
  entry: {
    entry1: './src/entry1.js',
    entry2: './src/entry2.js'
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
        use_: [
          path.resolve(__dirname, '../loaders/logger-loader.js')
        ]
      }
    ]
  },
  plugins: [
    new EmitPlugin()
  ]
};

// Create a compiler instance
const compiler = webpack(webpackOptions);

// Run the compiler
compiler.run((err, stats) => {
  if (err) {
    console.error('Compilation failed:', err);
    return;
  }

  console.log('Compilation successful!');
  console.log('Stats:', JSON.stringify(stats, null, 2));
});
