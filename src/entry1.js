// CommonJS import
const defaultA = require('./a.js');
const fnRes = defaultA()
console.log('CJS result:', fnRes)

// ESM import converted to CommonJS
const bModule = require('./b.js');
const defaultBB = bModule.default;
const name = bModule.name;
const bb = bModule.bb;
const moduleInfo = bModule.moduleInfo;
console.log('ESM default:', defaultBB());
console.log('ESM named:', name, bb());
console.log('ESM moduleInfo:', moduleInfo);

// Simple function
const getSum = (a, b) => {
  return a + b;
}

const x = getSum(1, 2);
console.log('Sum result:', x);

// Dynamic import using require (simulated)
console.log('Loading dynamic module...');
// In a real implementation, we would use dynamic import
// For now, we'll simulate it with a Promise and require
Promise.resolve().then(() => {
  const dynamicModule = require('./dynamic-module.js');
  console.log('Dynamic module loaded!');
  console.log('Dynamic default:', dynamicModule.default());
  console.log('Dynamic message:', dynamicModule.getDynamicMessage());
  console.log('Dynamic data:', dynamicModule.dynamicData);
}).catch(err => {
  console.error('Failed to load dynamic module:', err);
});
