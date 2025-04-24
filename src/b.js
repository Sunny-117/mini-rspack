// ESM module
export const name = 'b';

export function bb() {
  return 'bb';
}

export default function defaultBB() {
  return 'default bb';
}

// Add ESM-specific code
const moduleInfo = {
  type: 'module',
  version: '1.0.0'
};

export { moduleInfo };
