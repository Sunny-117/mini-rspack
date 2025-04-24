#![deny(clippy::all)]

use napi_derive::napi;
use serde::{Deserialize, Serialize};

mod compiler;
mod compilation;
mod module;
mod loader;
mod loader_runner;
mod plugin;
mod plugin_system;
mod utils;

use compiler::Compiler;

#[napi(object)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RspackOptions {
    pub mode: Option<String>,
    pub devtool: Option<bool>,
    pub watch: Option<bool>,
    pub context: Option<String>,
    pub entry: EntryOptions,
    pub output: OutputOptions,
    pub resolve: Option<ResolveOptions>,
    pub module: Option<ModuleOptions>,
    pub plugins: Option<Vec<String>>, // 简化为字符串列表
}

#[napi(object)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntryOptions {
    #[serde(flatten)]
    pub entries: std::collections::HashMap<String, String>,
}

#[napi(object)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutputOptions {
    pub path: String,
    pub filename: String,
}

#[napi(object)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResolveOptions {
    pub extensions: Option<Vec<String>>,
}

#[napi(object)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleOptions {
    pub rules: Option<Vec<RuleOptions>>,
}

#[napi(object)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleOptions {
    pub test: String,
    #[serde(rename = "use")]
    pub use_: Vec<String>,
}

#[napi]
pub fn rspack(options: RspackOptions) -> napi::Result<Compiler> {
    // Process command line arguments if needed

    // Create a new compiler instance
    let compiler = crate::compiler::create_compiler(options);

    // Return the compiler
    Ok(compiler)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        // Test implementation will be added later
    }
}
