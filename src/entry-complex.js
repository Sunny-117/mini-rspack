// 测试复杂ESM语法的入口文件 - 转换为CommonJS
const complexModule = require('./complex-esm.js');

// 解构导出
const ComplexClass = complexModule.default;
const complexValue = complexModule.complexValue;
const externalName = complexModule.externalName;
const asyncFunction = complexModule.asyncFunction;
const conditionalImport = complexModule.conditionalImport;

// 从b.js重新导出的内容
const name = complexModule.name;
const bb = complexModule.bb;

console.log('Complex ESM test started');

// 使用命名导出
console.log('Complex value:', complexValue);
console.log('External name:', externalName);

// 使用从b.js重新导出的内容
console.log('Re-exported name:', name);
console.log('Re-exported function result:', bb());

// 使用默认导出的类
const instance = new ComplexClass();
console.log('Instance type:', instance.getType());

// 使用静态方法
const anotherInstance = ComplexClass.createInstance();
console.log('Another instance type:', anotherInstance.getType());

// 使用异步函数
async function runAsync() {
  try {
    const result = await asyncFunction();
    console.log('Async function result:', result);

    // 测试条件动态导入 - true条件
    const dynamicModule = await conditionalImport(true);
    console.log('Conditional import (true):', {
      default: dynamicModule.default(),
      message: dynamicModule.getDynamicMessage(),
      data: dynamicModule.dynamicData
    });

    // 测试条件动态导入 - false条件
    const fallbackModule = await conditionalImport(false);
    console.log('Conditional import (false):', {
      default: fallbackModule.default(),
      message: fallbackModule.getDynamicMessage(),
      data: fallbackModule.dynamicData
    });

    console.log('Complex ESM test completed');
  } catch (error) {
    console.error('Error in async operations:', error);
  }
}

// 运行异步测试
runAsync();
