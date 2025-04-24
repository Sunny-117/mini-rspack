use std::path::Path;
use anyhow::Result;
use std::process::Command;
use std::fs;
use std::io::Write;
use std::env;
use std::collections::HashMap;

// Plugin上下文，包含编译信息
pub struct PluginContext {
    pub compiler_options: serde_json::Value,
    pub compilation: serde_json::Value,
    pub hooks: Vec<String>,
}

// Plugin系统，负责加载和执行插件
pub struct PluginSystem {
    pub plugins: Vec<Plugin>,
    pub context: PluginContext,
}

// 插件定义
pub struct Plugin {
    pub name: String,
    pub path: String,
    pub options: serde_json::Value,
}

impl PluginSystem {
    // 创建一个新的Plugin系统
    pub fn new(compiler_options: serde_json::Value) -> Self {
        // 创建上下文
        let context = PluginContext {
            compiler_options,
            compilation: serde_json::json!({}),
            hooks: vec![
                "beforeRun".to_string(),
                "run".to_string(),
                "beforeCompile".to_string(),
                "compile".to_string(),
                "make".to_string(),
                "afterCompile".to_string(),
                "emit".to_string(),
                "afterEmit".to_string(),
                "done".to_string(),
            ],
        };

        Self {
            plugins: Vec::new(),
            context,
        }
    }

    // 添加插件
    pub fn add_plugin(&mut self, name: &str, path: &str, options: serde_json::Value) {
        self.plugins.push(Plugin {
            name: name.to_string(),
            path: path.to_string(),
            options,
        });
    }

    // 从配置中加载插件
    pub fn load_plugins_from_config(&mut self, plugin_names: &[String], _base_dir: &str) -> Result<()> {
        for plugin_name in plugin_names {
            // 构建插件路径
            let plugin_path = if plugin_name.ends_with(".js") {
                // 如果已经是一个JS文件路径
                plugin_name.clone()
            } else {
                // 否则，假设它是一个内置插件或node_modules中的插件
                let current_dir = std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
                format!("{}/plugins/{}.js", current_dir.to_string_lossy(), plugin_name)
            };

            // 添加插件
            self.add_plugin(plugin_name, &plugin_path, serde_json::json!({}));
        }

        Ok(())
    }

    // 应用所有插件到指定的钩子
    pub fn apply_plugins(&self, hook_name: &str, hook_args: serde_json::Value) -> Result<serde_json::Value> {
        let mut result = hook_args.clone();

        println!("Applying plugins to hook: {}", hook_name);

        for plugin in &self.plugins {
            result = self.apply_plugin(plugin, hook_name, &result)?;
        }

        Ok(result)
    }

    // 应用单个插件到指定的钩子
    fn apply_plugin(&self, plugin: &Plugin, hook_name: &str, hook_args: &serde_json::Value) -> Result<serde_json::Value> {
        println!("Applying plugin: {} to hook: {}", plugin.name, hook_name);

        // 检查插件文件是否存在
        let plugin_path = Path::new(&plugin.path);
        if !plugin_path.exists() {
            println!("Plugin file not found: {}", plugin.path);
            return Ok(hook_args.clone());
        }

        // 准备插件的输入
        let plugin_input = serde_json::json!({
            "hook": hook_name,
            "args": hook_args,
            "options": plugin.options,
            "context": {
                "compiler": self.context.compiler_options,
                "compilation": self.context.compilation,
            },
        });

        // 创建临时文件来存储插件的输入
        let temp_dir = env::temp_dir();
        let input_file = temp_dir.join("plugin_input.json");
        let mut file = fs::File::create(&input_file)?;
        file.write_all(plugin_input.to_string().as_bytes())?;

        // 创建一个Node.js脚本来执行插件
        let runner_script = format!(
            r#"
            const fs = require('fs');
            const path = require('path');

            // 读取输入
            const inputData = JSON.parse(fs.readFileSync('{}'));

            // 加载插件
            const PluginClass = require('{}');

            // 创建插件实例
            const plugin = new PluginClass(inputData.options);

            // 创建模拟的compiler和compilation对象
            const compiler = {{
                options: inputData.context.compiler,
                hooks: {{}},
            }};

            const compilation = {{
                ...inputData.context.compilation,
                assets: {{}},
                hooks: {{}},
            }};

            // 为每个钩子创建tap方法
            const createTapMethod = (hookName) => {{
                return {{
                    tap: (name, callback) => {{
                        console.log(`Plugin '${{}}.js' tapped into hook: ${{hookName}}`);
                    }}
                }};
            }};

            // 添加所有钩子
            const hooks = [
                'beforeRun', 'run', 'beforeCompile', 'compile',
                'make', 'afterCompile', 'emit', 'afterEmit', 'done'
            ];

            hooks.forEach(hook => {{
                compiler.hooks[hook] = createTapMethod(hook);
                compilation.hooks[hook] = createTapMethod(hook);
            }});

            // 应用插件
            if (typeof plugin.apply === 'function') {{
                plugin.apply(compiler);
            }}

            // 如果是当前钩子，执行相应的处理
            if (inputData.hook === 'emit' && typeof plugin.emit === 'function') {{
                const result = plugin.emit(compilation, inputData.args);
                console.log(JSON.stringify({{ result }}));
            }} else if (inputData.hook === 'done' && typeof plugin.done === 'function') {{
                const result = plugin.done(compilation, inputData.args);
                console.log(JSON.stringify({{ result }}));
            }} else {{
                // 返回原始参数
                console.log(JSON.stringify({{ result: inputData.args }}));
            }}
            "#,
            input_file.to_string_lossy(),
            plugin_path.to_string_lossy()
        );

        let runner_file = temp_dir.join("plugin_runner.js");
        let mut file = fs::File::create(&runner_file)?;
        file.write_all(runner_script.as_bytes())?;

        // 执行Node.js脚本
        let output = Command::new("node")
            .arg(&runner_file)
            .output()?;

        // 清理临时文件
        fs::remove_file(input_file)?;
        fs::remove_file(runner_file)?;

        // 解析输出
        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);

            // 解析JSON输出
            if let Ok(json) = serde_json::from_str::<serde_json::Value>(&stdout) {
                if let Some(result) = json.get("result") {
                    return Ok(result.clone());
                }
            }

            // 如果没有JSON输出，返回原始参数
            Ok(hook_args.clone())
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            println!("Plugin execution failed: {}", stderr);
            Ok(hook_args.clone())
        }
    }
}

// 应用插件到编译过程
pub fn apply_plugins_to_compilation(
    compilation: &mut crate::compilation::Compilation,
    plugin_names: &[String],
    base_dir: &str,
) -> Result<()> {
    // 创建插件系统
    let mut plugin_system = PluginSystem::new(serde_json::to_value(&compilation.options)?);

    // 加载插件
    plugin_system.load_plugins_from_config(plugin_names, base_dir)?;

    // 应用emit钩子
    let assets_json = serde_json::to_value(&compilation.assets)?;
    let updated_assets = plugin_system.apply_plugins("emit", assets_json)?;

    // 更新assets
    if let Ok(assets_map) = serde_json::from_value::<HashMap<String, String>>(updated_assets) {
        compilation.assets = assets_map;
    }

    Ok(())
}
