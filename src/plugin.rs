use napi_derive::napi;
use std::collections::HashMap;

// Define a trait for plugins
pub trait Plugin {
    fn apply(&self, compiler: &mut crate::compiler::Compiler);
    fn name(&self) -> &str;
}

// Define a trait for compilation plugins
pub trait CompilationPlugin {
    fn apply(&self, compilation: &mut crate::compilation::Compilation);
    fn name(&self) -> &str;
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

// Plugin implementation
pub struct JsPlugin {
    pub name: String,
    pub path: String,
}

impl Plugin for JsPlugin {
    fn apply(&self, compiler: &mut crate::compiler::Compiler) {
        compiler.hooks.emit.tap(&self.name);
        println!("Applying plugin: {} from {}", self.name, self.path);
    }

    fn name(&self) -> &str {
        &self.name
    }
}

impl CompilationPlugin for JsPlugin {
    fn apply(&self, compilation: &mut crate::compilation::Compilation) {
        compilation.hooks.emit.tap(&self.name);

        // In a real implementation, we would load and execute the JS plugin
        // For now, we'll simulate the plugin behavior based on the plugin name

        if self.name == "HtmlWebpackPlugin" {
            // Simulate HtmlWebpackPlugin
            let html_content = format!(r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8">
    <title>Mini Rspack App</title>
</head>
<body>
    <div id="app"></div>
    <script src="{}.js"></script>
</body>
</html>"#, compilation.options.entry.entries.keys().next().unwrap_or(&"main".to_string()));

            compilation.assets.insert("index.html".to_string(), html_content);
        } else if self.name == "MiniCssExtractPlugin" {
            // Simulate MiniCssExtractPlugin
            // In a real implementation, this would extract CSS from JS files
            // For now, we'll just create an empty CSS file
            compilation.assets.insert("styles.css".to_string(), "".to_string());
        } else if self.name == "BannerPlugin" {
            // Simulate BannerPlugin
            // Add a banner to all JS files
            let banner = "/* This file is generated by mini-rspack */\n";

            for (filename, content) in compilation.assets.iter_mut() {
                if filename.ends_with(".js") {
                    *content = format!("{}{}", banner, content);
                }
            }
        }
    }

    fn name(&self) -> &str {
        &self.name
    }
}

// EmitPlugin implementation (similar to the JS version)
pub struct EmitPlugin;

impl Plugin for EmitPlugin {
    fn apply(&self, compiler: &mut crate::compiler::Compiler) {
        compiler.hooks.emit.tap("EmitPlugin");
    }

    fn name(&self) -> &str {
        "EmitPlugin"
    }
}

impl CompilationPlugin for EmitPlugin {
    fn apply(&self, compilation: &mut crate::compilation::Compilation) {
        compilation.hooks.emit.tap("EmitPlugin");

        // Add a list of assets to the output
        let assets_list = compilation.assets.keys().cloned().collect::<Vec<_>>().join("\n");
        compilation.assets.insert("assets.md".to_string(), assets_list);
    }

    fn name(&self) -> &str {
        "EmitPlugin"
    }
}

// This function would be called from JS to register the plugin
pub fn register_plugin(compilation: &mut crate::compilation::Compilation, plugin_name: &str) {
    match plugin_name {
        "EmitPlugin" => {
            let plugin = EmitPlugin;
            CompilationPlugin::apply(&plugin, compilation);
        },
        "HtmlWebpackPlugin" => {
            let plugin = JsPlugin {
                name: "HtmlWebpackPlugin".to_string(),
                path: "../plugins/html-webpack-plugin.js".to_string(),
            };
            CompilationPlugin::apply(&plugin, compilation);
        },
        "MiniCssExtractPlugin" => {
            let plugin = JsPlugin {
                name: "MiniCssExtractPlugin".to_string(),
                path: "../plugins/mini-css-extract-plugin.js".to_string(),
            };
            CompilationPlugin::apply(&plugin, compilation);
        },
        "BannerPlugin" => {
            let plugin = JsPlugin {
                name: "BannerPlugin".to_string(),
                path: "../plugins/banner-plugin.js".to_string(),
            };
            CompilationPlugin::apply(&plugin, compilation);
        },
        _ => {
            println!("Unknown plugin: {}", plugin_name);
        }
    }
}
