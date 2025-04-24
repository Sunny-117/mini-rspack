const path = require('path');

module.exports = {
  mode: 'development',
  entry: {
    entry1: './src/entry1.js',
    entry2: './src/entry2.js',
    'entry-complex': './src/entry-complex.js'
  },
  output: {
    path: path.resolve(__dirname, 'dist'),
    filename: '[name].js'
  },
  module: {
    rules: [
      {
        test: /\.js$/,
        use: [
          {
            loader: path.resolve(__dirname, 'loaders/logger-loader.js')
          }
        ]
      }
    ]
  },
  resolve: {
    extensions: ['.js', '.json']
  }
};
