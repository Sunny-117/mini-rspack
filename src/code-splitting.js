// 测试代码分割功能
console.log('Code splitting test started');

// 动态导入模块
function loadModule(name) {
  console.log(`Loading module: ${name}`);
  
  if (name === 'dynamic') {
    return import('./dynamic-module.js')
      .then(module => {
        console.log('Dynamic module loaded');
        return {
          type: 'dynamic',
          default: module.default(),
          message: module.getDynamicMessage(),
          data: module.dynamicData
        };
      });
  } else if (name === 'complex') {
    return import('./complex-esm.js')
      .then(module => {
        console.log('Complex module loaded');
        return {
          type: 'complex',
          class: new module.default(),
          value: module.complexValue,
          async: module.asyncFunction()
        };
      });
  } else {
    return Promise.reject(new Error(`Unknown module: ${name}`));
  }
}

// 加载多个模块
Promise.all([
  loadModule('dynamic'),
  loadModule('complex')
])
  .then(results => {
    console.log('All modules loaded');
    console.log('Results:', JSON.stringify(results, null, 2));
    console.log('Code splitting test completed');
  })
  .catch(error => {
    console.error('Error loading modules:', error);
  });
