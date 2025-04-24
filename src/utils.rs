use std::path::{Path, PathBuf};

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

// Check if a module is an ESM module
fn is_esm_module(source: &str) -> bool {
    source.contains("export ") || source.contains("import ")
}

// Transform ESM module to CommonJS
fn transform_esm_to_cjs(source: &str) -> String {
    let mut transformed = source.to_string();
    
    // Replace export default with module.exports.default =
    let default_export_regex = regex::Regex::new(r"export\s+default\s+(.+?)(?:;|\n|$)").unwrap();
    transformed = default_export_regex.replace_all(&transformed, "module.exports.default = $1;").to_string();
    
    // Replace named exports with exports.name = value
    let named_export_regex = regex::Regex::new(r"export\s+(const|let|var)\s+(\w+)\s*=\s*(.+?)(?:;|\n|$)").unwrap();
    transformed = named_export_regex.replace_all(&transformed, "exports.$2 = $3;").to_string();
    
    // Replace export function with exports.function =
    let function_export_regex = regex::Regex::new(r"export\s+function\s+(\w+)").unwrap();
    transformed = function_export_regex.replace_all(&transformed, "exports.$1 = function $1").to_string();
    
    // Replace export { x } with exports.x = x
    let export_object_regex = regex::Regex::new(r"export\s+\{\s*(.+?)\s*\}").unwrap();
    transformed = export_object_regex.replace_all(&transformed, |caps: &regex::Captures| {
        let exports = caps.get(1).unwrap().as_str();
        let mut result = String::new();
        for export in exports.split(',') {
            let export = export.trim();
            result.push_str(&format!("exports.{} = {};\n", export, export));
        }
        result
    }).to_string();
    
    transformed
}

// Generate the bundle source code
pub fn generate_bundle_source(chunk: &crate::compilation::Chunk) -> String {
    // Create a map of module paths for resolving dependencies
    let mut module_path_map = std::collections::HashMap::new();
    
    // Collect all modules and their dependencies
    for module in &chunk.modules {
        for dep in &module.dependencies {
            let relative_path = dep.dep_module_id.clone();
            let full_path = dep.dep_module_path.clone();
            module_path_map.insert(relative_path, full_path);
        }
    }
    
    // Generate module code with unique IDs
    let modules_code = chunk.modules.iter()
        .map(|module| {
            // Process the source code to replace require paths
            let mut processed_source = module.source.clone();
            
            // Replace require('./path') with require('./resolved/path')
            let require_regex = regex::Regex::new(r#"require\(['"](\./[^'"]+)['"]\)"#).unwrap();
            processed_source = require_regex.replace_all(&processed_source, |caps: &regex::Captures| {
                let path = caps.get(1).unwrap().as_str();
                format!("require('./src/{}')", path.trim_start_matches("./"))
            }).to_string();
            
            // Check if this is an ESM module
            let is_esm = is_esm_module(&module.source);
            
            // Transform ESM to CommonJS if needed
            if is_esm {
                // Replace import statements with require
                let import_regex = regex::Regex::new(r#"import\s+(\w+)\s*,?\s*\{\s*([^}]+)\s*\}\s+from\s+['"](\./[^'"]+)['"]\s*;"#).unwrap();
                processed_source = import_regex.replace_all(&processed_source, |caps: &regex::Captures| {
                    let default_import = caps.get(1).unwrap().as_str();
                    let named_imports = caps.get(2).unwrap().as_str();
                    let path = caps.get(3).unwrap().as_str();
                    
                    let mut result = format!("const _module = require('./src/{}')\n", path.trim_start_matches("./"));
                    result.push_str(&format!("const {} = _module.default;\n", default_import));
                    
                    for named_import in named_imports.split(',') {
                        let named_import = named_import.trim();
                        result.push_str(&format!("const {} = _module.{};\n", named_import, named_import));
                    }
                    
                    result
                }).to_string();
                
                // Transform the rest of ESM syntax to CommonJS
                processed_source = transform_esm_to_cjs(&processed_source);
            }
            
            // Add ESM wrapper if needed
            let module_code = if is_esm {
                format!(
                    r#"
        "{}": function(module, exports, require) {{
            // ESM Module
            __webpack_require__.r(exports);
            {}
        }}"#,
                    module.id,
                    processed_source
                )
            } else {
                format!(
                    r#"
        "{}": function(module, exports, require) {{
            {}
        }}"#,
                    module.id,
                    processed_source
                )
            };
            
            module_code
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
