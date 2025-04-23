module.exports = function (sourceCode, name, modulePath) {
  console.log("Logger loader processing file:", { name, modulePath });
  return sourceCode;
}
