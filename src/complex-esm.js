// 复杂的ESM语法测试 - 转换为CommonJS

// 命名导出
const complexValue = 'complex value';

// 重命名导出
const internalName = 'internal';
const externalName = internalName;

// 聚合导出 - 使用require
const bModule = require('./b.js');

// 默认导出
class ComplexClass {
  constructor() {
    this.type = 'complex';
  }

  getType() {
    return this.type;
  }

  static createInstance() {
    return new ComplexClass();
  }
}

// 异步函数
async function asyncFunction() {
  // 模拟异步操作
  return new Promise(resolve => {
    setTimeout(() => {
      resolve('async result');
    }, 100);
  });
}

// 条件动态导入 - 使用require模拟
function conditionalImport(condition) {
  return Promise.resolve().then(() => {
    if (condition) {
      const dynamicModule = require('./dynamic-module.js');
      return dynamicModule;
    } else {
      const aModule = require('./a.js');
      return {
        default: aModule,
        getDynamicMessage: () => 'Fallback message',
        dynamicData: { name: 'fallback', version: '0.0.1' }
      };
    }
  });
}

// 将所有导出合并到一个对象中
module.exports = {
  complexValue,
  externalName,
  // 将b.js的导出展开
  name: bModule.name,
  bb: bModule.bb,
  moduleInfo: bModule.moduleInfo,
  // 其他导出
  default: ComplexClass,
  ComplexClass,
  asyncFunction,
  conditionalImport
};
