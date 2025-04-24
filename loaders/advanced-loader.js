/**
 * 高级loader示例，演示如何使用loader上下文和选项
 */
module.exports = function(source) {
  // 获取loader上下文
  const context = this || {};
  
  // 记录loader执行信息
  console.log(`Advanced loader processing file: ${context.resourcePath || 'unknown'}`);
  
  // 获取loader选项
  const options = context.options || {};
  const prefix = options.prefix || '/* Advanced Loader */';
  const suffix = options.suffix || '';
  
  // 处理源代码
  let result = source;
  
  // 添加前缀
  result = `${prefix}\n${result}`;
  
  // 添加后缀
  if (suffix) {
    result = `${result}\n${suffix}`;
  }
  
  // 添加处理信息
  result = `${result}\n// Processed by advanced-loader at ${new Date().toISOString()}`;
  
  // 返回处理后的代码
  return result;
};
