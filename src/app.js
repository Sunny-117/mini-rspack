// Import CSS
import './style.css';

// Import logo
import logoUrl from './logo.svg';

// Import HTML template
import template from './template.html';

// Import modules
import { name, bb } from './b.js';
import defaultA from './a.js';

// Create a logo element
const logo = document.createElement('img');
logo.src = logoUrl;
logo.alt = 'Mini Rspack Logo';
logo.width = 100;
document.querySelector('#app').prepend(logo);

// Add event listener to the button
document.getElementById('loadButton').addEventListener('click', async () => {
  try {
    // Dynamic import
    const dynamicModule = await import('./dynamic-module.js');
    
    // Display the result
    const resultDiv = document.getElementById('result');
    resultDiv.innerHTML = `
      <h2>Dynamic Module Loaded!</h2>
      <p>Default: ${dynamicModule.default()}</p>
      <p>Message: ${dynamicModule.getDynamicMessage()}</p>
      <p>Data: ${JSON.stringify(dynamicModule.dynamicData)}</p>
      <p>Module B: ${name}, ${bb()}</p>
      <p>Module A: ${defaultA()}</p>
    `;
  } catch (error) {
    console.error('Failed to load dynamic module:', error);
    document.getElementById('result').textContent = `Error: ${error.message}`;
  }
});

// Log some information
console.log('App initialized');
console.log('Module A:', defaultA());
console.log('Module B:', name, bb());
