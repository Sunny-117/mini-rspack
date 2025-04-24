use std::path::{Path, PathBuf};
use std::collections::HashSet;

use anyhow::{Result, anyhow};

// Convert Windows backslashes to forward slashes
pub fn to_unix_path<P: AsRef<Path>>(path: P) -> String {
    path.as_ref()
        .to_string_lossy()
        .replace('\\', "/")
}

// Try to resolve a module path with different extensions
pub fn try_extensions(module_path: &Path, extensions: &[String]) -> Result<PathBuf> {
    // First try the path as-is (for paths that already have an extension)
    if module_path.exists() {
        return Ok(module_path.to_path_buf());
    }

    // Then try with each extension
    for ext in extensions {
        let path_with_ext = module_path.with_extension(ext.trim_start_matches('.'));
        if path_with_ext.exists() {
            return Ok(path_with_ext);
        }
    }

    Err(anyhow!("Module not found: {}", module_path.display()))
}

// Generate the bundle source code
pub fn generate_bundle_source(chunk: &crate::compilation::Chunk) -> String {
    // Collect all modules from the chunk and its dependencies
    let mut all_modules = Vec::new();
    let mut processed_ids = HashSet::new();
    
    // Add the entry module
    all_modules.push(&chunk.entry_module);
    processed_ids.insert(chunk.entry_module.id.clone());
    
    // Add all other modules from the chunk
    for module in &chunk.modules {
        if !processed_ids.contains(&module.id) {
            all_modules.push(module);
            processed_ids.insert(module.id.clone());
        }
    }
    
    // Generate module code with unique IDs
    let modules_code = all_modules.iter()
        .map(|module| {
            // Process the source code to replace require paths
            let mut processed_source = module.source.clone();
            
            // Replace require('./path') with __webpack_require__('./path')
            let require_regex = regex::Regex::new(r#"require\(['"](\./[^'"]+)['"]\)"#).unwrap();
            processed_source = require_regex.replace_all(&processed_source, |caps: &regex::Captures| {
                let path = caps.get(1).unwrap().as_str();
                format!("__webpack_require__('./src/{}')", path.trim_start_matches("./"))
            }).to_string();
            
            format!(
                r#"
        "{}": function(module, exports, __webpack_require__) {{
            {}
        }}"#,
                module.id,
                processed_source
            )
        })
        .collect::<Vec<_>>()
        .join(",");

    format!(
        r#"
(() => {{
    // webpackBootstrap
    var __webpack_modules__ = {{{}}};
    
    // The module cache
    var __webpack_module_cache__ = {{}};
    
    // The require function
    function __webpack_require__(moduleId) {{
        // Check if module is in cache
        var cachedModule = __webpack_module_cache__[moduleId];
        
        if (cachedModule !== undefined) {{
            return cachedModule.exports;
        }}
        
        // Create a new module (and put it into the cache)
        var module = __webpack_module_cache__[moduleId] = {{
            id: moduleId,
            loaded: false,
            exports: {{}}
        }};
        
        // Execute the module function
        __webpack_modules__[moduleId](module, module.exports, __webpack_require__);
        
        // Flag the module as loaded
        module.loaded = true;
        
        // Return the exports of the module
        return module.exports;
    }}
    
    // Define __esModule on exports
    __webpack_require__.r = function(exports) {{
        if(typeof Symbol !== 'undefined' && Symbol.toStringTag) {{
            Object.defineProperty(exports, Symbol.toStringTag, {{ value: 'Module' }});
        }}
        Object.defineProperty(exports, '__esModule', {{ value: true }});
    }};
    
    // Create a fake namespace object
    __webpack_require__.t = function(value, mode) {{
        if(mode & 1) value = __webpack_require__(value);
        if(mode & 8) return value;
        if((mode & 4) && typeof value === 'object' && value && value.__esModule) return value;
        var ns = Object.create(null);
        __webpack_require__.r(ns);
        Object.defineProperty(ns, 'default', {{ enumerable: true, value: value }});
        if(mode & 2 && typeof value != 'string') for(var key in value) __webpack_require__.d(ns, key, function(key) {{ return value[key]; }}.bind(null, key));
        return ns;
    }};
    
    // Define getter function for harmony exports
    __webpack_require__.d = function(exports, name, getter) {{
        if(!__webpack_require__.o(exports, name)) {{
            Object.defineProperty(exports, name, {{ enumerable: true, get: getter }});
        }}
    }};
    
    // Define property getter
    __webpack_require__.o = function(obj, prop) {{ return Object.prototype.hasOwnProperty.call(obj, prop); }};
    
    // Define export property
    __webpack_require__.s = "";
    
    // Load entry module and return exports
    return __webpack_require__("{}");
}})();
"#,
        modules_code,
        chunk.entry_module.id
    )
}
