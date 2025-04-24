/**
 * A simplified babel-loader that transforms ES6+ code to ES5
 * In a real implementation, this would use Babel to transform the code
 */
module.exports = function(sourceCode, name, modulePath) {
  console.log(`Babel loader processing file: ${modulePath}`);
  
  // Simple transformations (not a real Babel implementation)
  let transformedCode = sourceCode;
  
  // Transform arrow functions to regular functions
  transformedCode = transformedCode.replace(/\(([^)]*)\)\s*=>\s*{/g, 'function($1) {');
  
  // Transform const/let to var
  transformedCode = transformedCode.replace(/const\s+/g, 'var ');
  transformedCode = transformedCode.replace(/let\s+/g, 'var ');
  
  // Transform template literals to string concatenation
  transformedCode = transformedCode.replace(/`(.*?)`/g, function(match, p1) {
    return "'" + p1.replace(/\${(.*?)}/g, "' + $1 + '") + "'";
  });
  
  console.log('Babel transformation complete');
  return transformedCode;
}
