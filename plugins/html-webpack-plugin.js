const fs = require('fs');
const path = require('path');

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
      
      try {
        console.log(compilation)
        // Get the entry name
        const entryNames = Object.keys(compilation.options.entry.entries);
        const entryName = entryNames[0] || 'main';
        // Generate HTML content
        const htmlContent = this.generateHtml(entryName);

        // Add the HTML file to the assets
        compilation.assets[this.options.filename] = htmlContent;
        
        console.log(`HtmlWebpackPlugin: Generated ${this.options.filename}`);
      } catch (error) {
        console.log(error, 'error HtmlWebpackPlugin')
      }
    });
  }
  
  generateHtml(entryName) {
    const defaultHtml = `<!DOCTYPE html>
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

    if (this.options.template) {
      const templatePath = path.isAbsolute(this.options.template)
        ? this.options.template
        : path.resolve(process.cwd(), this.options.template);
      if (!fs.existsSync(templatePath)) {
        console.warn(`HtmlWebpackPlugin: template not found at ${templatePath}, using default template.`);
        return defaultHtml;
      }
      let templateContent = fs.readFileSync(templatePath, 'utf8');
      templateContent = templateContent.replace(/<%=\s*htmlWebpackPlugin.options.title\s*%>/g, this.options.title);

      if (this.options.inject) {
        const scriptTag = `<script src="${entryName}.js"></script>`;
        if (templateContent.includes('</body>')) {
          templateContent = templateContent.replace('</body>', `  ${scriptTag}\n</body>`);
        } else {
          templateContent += `\n${scriptTag}`;
        }
      }

      return templateContent;
    }

    return defaultHtml;
  }
}

module.exports = HtmlWebpackPlugin;
