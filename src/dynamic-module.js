// 这是一个将被动态导入的模块
const dynamicData = {
  name: 'dynamic-module',
  version: '1.0.0'
};

function getDynamicMessage() {
  return 'This message is from a dynamically imported module!';
}

function dynamicDefault() {
  return 'Dynamic default export';
}

// 使用CommonJS导出，避免ESM转换问题
module.exports = {
  dynamicData,
  getDynamicMessage,
  default: dynamicDefault
};
