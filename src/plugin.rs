use napi_derive::napi;
use std::collections::HashMap;

// Define a trait for plugins
pub trait Plugin {
    fn apply(&self, compiler: &mut crate::compiler::Compiler);
}

// Hook system similar to tapable in JS
#[napi(object)]
#[derive(Debug, Clone)]
pub struct SyncHook {
    pub name: String,
    pub taps: Vec<String>,
}

impl SyncHook {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            taps: Vec::new(),
        }
    }

    pub fn tap(&mut self, name: &str) {
        self.taps.push(name.to_string());
    }

    pub fn call(&self, _assets: Option<&mut HashMap<String, String>>) {
        // In a real implementation, this would call the registered callbacks
        println!("Hook '{}' called with {} taps", self.name, self.taps.len());

        // For now, we'll just print the taps
        for tap in &self.taps {
            println!("  - Tap: {}", tap);
        }
    }
}

// EmitPlugin implementation (similar to the JS version)
pub struct EmitPlugin;

impl Plugin for EmitPlugin {
    fn apply(&self, compiler: &mut crate::compiler::Compiler) {
        compiler.hooks.emit.tap("emit");
    }
}

// This function would be called from JS to register the plugin
pub fn register_emit_plugin(compiler: &mut crate::compiler::Compiler) {
    let plugin = EmitPlugin;
    plugin.apply(compiler);
}
