const defaultA = require('./a.js');
const fnRes = defaultA()
console.log(fnRes)
const getSum = (a, b) => {
  return a + b;
}

const x = getSum(1, 2);
console.log(x);
