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

// Generate the bundle source code
pub fn generate_bundle_source(chunk: &crate::compilation::Chunk) -> String {
    let modules_code = chunk.modules.iter()
        .map(|module| {
            format!(
                r#"
        "{}":(module,exports,require)=>{{
            {}
        }}
                "#,
                module.id,
                module.source
            )
        })
        .collect::<Vec<_>>()
        .join(",");

    format!(
        r#"
(() => {{
    // webpackBootstrap
    var __webpack_modules__ = ({{
{}
    }});
    
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
            exports: {{}}
        }};
        
        // Execute the module function
        __webpack_modules__[moduleId](module, module.exports, __webpack_require__);
        
        // Return the exports of the module
        return module.exports;
    }}
    
    // Execute the entry module
    var __webpack_exports__ = __webpack_require__("{}");
    
}})();
"#,
        modules_code,
        chunk.entry_module.id
    )
}
