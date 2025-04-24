/**
 * A simplified HtmlWebpackPlugin that generates an HTML file
 * In a real implementation, this would use a template engine
 */
class HtmlWebpackPlugin {
  constructor(options = {}) {
    this.options = Object.assign({
      title: 'Mini Rspack App',
      filename: 'index.html',
      template: null,
      inject: true,
      minify: false,
    }, options);
  }
  
  apply(compiler) {
    // Hook into the emit event
    compiler.hooks.emit.tap('HtmlWebpackPlugin', (compilation) => {
      console.log('HtmlWebpackPlugin: Generating HTML file');
      
      // Get the entry name
      const entryNames = Object.keys(compilation.options.entry.entries);
      const entryName = entryNames[0] || 'main';
      
      // Generate HTML content
      const htmlContent = this.generateHtml(entryName);
      
      // Add the HTML file to the assets
      compilation.assets[this.options.filename] = htmlContent;
      
      console.log(`HtmlWebpackPlugin: Generated ${this.options.filename}`);
    });
  }
  
  generateHtml(entryName) {
    // In a real implementation, this would use a template engine
    return `<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8">
    <title>${this.options.title}</title>
    <link rel="stylesheet" href="styles.css">
</head>
<body>
    <div id="app"></div>
    <script src="${entryName}.js"></script>
</body>
</html>`;
  }
}

module.exports = HtmlWebpackPlugin;
