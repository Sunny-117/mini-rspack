// ESM module converted to CommonJS
const name = 'b';

function bb() {
  return 'bb';
}

function defaultBB() {
  return 'default bb';
}

// Add ESM-specific code
const moduleInfo = {
  type: 'module',
  version: '1.0.0'
};

// Use CommonJS exports
module.exports = {
  name,
  bb,
  moduleInfo,
  default: defaultBB
};
