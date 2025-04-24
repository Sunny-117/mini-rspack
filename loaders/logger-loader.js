/**
 * A simple logger loader that logs information about the module being processed
 * This is useful for debugging and understanding the loader chain
 */
module.exports = function (sourceCode, name, modulePath) {
  console.log(`Running loader: ${__filename} on module: ${modulePath}`);
  
  // Add a comment to the source code to indicate that it was processed by this loader
  const processedCode = `// Processed by logger-loader: ${new Date().toISOString()}\n${sourceCode}`;
  
  return processedCode;
}
