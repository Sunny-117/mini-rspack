// CommonJS import
const defaultA = require('./a.js');
const fnRes = defaultA()
console.log('CJS result:', fnRes)

// ESM import
import defaultBB, { name, bb, moduleInfo } from './b.js';
console.log('ESM default:', defaultBB());
console.log('ESM named:', name, bb());
console.log('ESM moduleInfo:', moduleInfo);

// Simple function
const getSum = (a, b) => {
  return a + b;
}

const x = getSum(1, 2);
console.log('Sum result:', x);
