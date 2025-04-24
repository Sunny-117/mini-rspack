/**
 * A simplified file-loader that transforms files to URLs
 * In a real implementation, this would copy the file to the output directory
 */
module.exports = function(sourceCode, name, modulePath) {
  console.log(`File loader processing file: ${modulePath}`);

  // Get the file name from the module path
  const path = require('path');
  const fileName = path.basename(modulePath || 'unknown');

  // Generate a URL for the file (in a real implementation, this would be a hash)
  const fileHash = Date.now().toString(36);
  const outputFileName = `${fileHash}-${fileName}`;
  const publicPath = '/assets/';
  const fileUrl = `${publicPath}${outputFileName}`;

  // In a real implementation, we would copy the file to the output directory
  // fs.copyFileSync(modulePath, path.join(outputPath, outputFileName));

  // Generate JavaScript module that exports the URL
  const jsModule = `
// File Module
module.exports = '${fileUrl}';
module.exports.default = '${fileUrl}';
`;

  console.log(`File transformation complete: ${fileUrl}`);
  return jsModule;
}
