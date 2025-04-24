/**
 * A simplified html-loader that transforms HTML to JS
 * In a real implementation, this would parse the HTML and handle imports, src attributes, etc.
 */
module.exports = function(sourceCode, name, modulePath) {
  console.log(`HTML loader processing file: ${modulePath}`);
  
  // Escape special characters
  const escapedHTML = sourceCode
    .replace(/\\/g, '\\\\')
    .replace(/'/g, "\\'")
    .replace(/\n/g, '\\n');
  
  // Generate JavaScript module that exports the HTML
  const jsModule = `
// HTML Module
const html = '${escapedHTML}';

// Export the HTML string
module.exports = html;
module.exports.default = html;

// Export a function to add the HTML to the DOM
module.exports.addToDOM = function(selector) {
  const container = document.querySelector(selector || 'body');
  if (container) {
    container.innerHTML = html;
  }
};
`;
  
  console.log('HTML transformation complete');
  return jsModule;
}
