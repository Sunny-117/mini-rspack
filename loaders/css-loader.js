/**
 * A simplified css-loader that transforms CSS to JS
 * In a real implementation, this would parse the CSS and handle imports, url(), etc.
 */
module.exports = function(sourceCode, name, modulePath) {
  console.log(`CSS loader processing file: ${modulePath}`);
  
  // Escape special characters
  const escapedCSS = sourceCode
    .replace(/\\/g, '\\\\')
    .replace(/'/g, "\\'")
    .replace(/\n/g, '\\n');
  
  // Generate JavaScript module that exports the CSS
  const jsModule = `
// CSS Module
const css = '${escapedCSS}';

// Add CSS to the DOM
const styleEl = document.createElement('style');
styleEl.textContent = css;
document.head.appendChild(styleEl);

module.exports = {
  // Export the CSS string
  toString: function() { return css; },
  
  // Export a function to add the CSS to the DOM
  addToDOM: function() {
    const styleEl = document.createElement('style');
    styleEl.textContent = css;
    document.head.appendChild(styleEl);
  }
};
`;
  
  console.log('CSS transformation complete');
  return jsModule;
}
