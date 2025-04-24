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
    pub fn load_plugins_from_config(&mut self, plugin_names: &[String], base_dir: &str) -> Result<()> {
        for plugin_name in plugin_names {
            // 构建插件路径
            let plugin_path = if plugin_name.ends_with(".js") {
                // 如果已经是一个JS文件路径
                plugin_name.clone()
            } else {
                // 否则，尝试多种可能的插件路径
                let base_path = std::path::Path::new(base_dir);

                // 尝试不同的命名约定
                let kebab_case_name = plugin_name.replace("Plugin", "-plugin").replace("Webpack", "-webpack").to_lowercase();

                // 可能的路径列表
                let possible_paths = vec![
                    // 1. plugins/emit-plugin.js (kebab-case)
                    base_path.join("plugins").join(format!("{}.js", kebab_case_name)),
                    // 2. plugins/EmitPlugin.js (PascalCase)
                    base_path.join("plugins").join(format!("{}.js", plugin_name)),
                    // 3. ./plugins/emit-plugin.js (相对路径)
                    std::path::Path::new("./plugins").join(format!("{}.js", kebab_case_name)),
                    // 4. node_modules/emit-plugin (npm包)
                    base_path.join("node_modules").join(kebab_case_name),
                ];

                // 查找第一个存在的路径
                let found_path = possible_paths.iter()
                    .find(|path| path.exists())
                    .map(|path| path.to_string_lossy().to_string());

                // 如果找到了路径，使用它；否则使用默认路径
                found_path.unwrap_or_else(|| {
                    println!("Warning: Plugin file not found for {}, using default path", plugin_name);
                    format!("{}/plugins/{}.js", base_dir, plugin_name.to_lowercase().replace("plugin", "-plugin"))
                })
            };

            println!("Loading plugin: {} from path: {}", plugin_name, plugin_path);

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

        // 创建一个 Node.js 脚本来执行插件
        let runner_script = r#"
const fs = require('fs');
const path = require('path');

// 读取命令行参数
const inputFilePath = process.argv[2];
const pluginPath = process.argv[3];

try {
    // 读取输入
    const inputData = JSON.parse(fs.readFileSync(inputFilePath, 'utf8'));

    // 获取插件名称
    const pluginName = path.basename(pluginPath, '.js');
    console.log(`Executing plugin: ${pluginName}`);

    // 加载插件
    const PluginClass = require(pluginPath);
    console.log(`Plugin class loaded: ${typeof PluginClass}`);

    // 创建插件实例
    const plugin = new PluginClass(inputData.options);
    console.log(`Plugin instance created: ${typeof plugin}`);

    // 创建模拟的 compiler 和 compilation 对象
    const compiler = {
        options: inputData.context.compiler,
        hooks: {},
    };

    // 如果是 emit 钩子，将 assets 添加到 compilation 对象
    const assets = inputData.hook === 'emit' ? {...inputData.args} : {};

    const compilation = {
        ...inputData.context.compilation,
        assets: assets,
        hooks: {},
    };

    // 存储回调函数
    const callbacks = {};

    // 为每个钩子创建 tap 方法
    const createTapMethod = (hookName) => {
        return {
            tap: (name, callback) => {
                console.log(`Plugin '${pluginName}' tapped into hook: ${hookName} with name: ${name}`);
                // 存储回调函数
                if (!callbacks[hookName]) {
                    callbacks[hookName] = [];
                }
                callbacks[hookName].push({ name, callback });
            }
        };
    };

    // 添加所有钩子
    const hooks = [
        'beforeRun', 'run', 'beforeCompile', 'compile',
        'make', 'afterCompile', 'emit', 'afterEmit', 'done'
    ];

    hooks.forEach(hook => {
        compiler.hooks[hook] = createTapMethod(hook);
        compilation.hooks[hook] = createTapMethod(hook);
    });

    // 应用插件
    if (typeof plugin.apply === 'function') {
        plugin.apply(compiler);
    } else {
        console.log(`Warning: Plugin ${pluginName} does not have an apply method`);
    }

    // 如果是当前钩子，执行相应的回调函数
    if (inputData.hook === 'emit' && callbacks['emit'] && callbacks['emit'].length > 0) {
        console.log(`Executing ${callbacks['emit'].length} callbacks for hook: emit`);

        // 执行所有注册到 emit 钩子的回调函数
        for (const { name, callback } of callbacks['emit']) {
            console.log(`Executing callback for hook: emit, name: ${name}`);
            try {
                // 执行回调函数，传入 assets 对象
                callback(assets);
            } catch (error) {
                console.error(`Error executing callback for hook: emit, name: ${name}`, error);
            }
        }

        // 返回更新后的 assets 对象
        console.log(`Assets after plugin execution: ${Object.keys(assets).join(', ')}`);
        console.log(JSON.stringify({ result: assets }));
    } else {
        // 返回原始参数
        console.log(`No callbacks found for hook: ${inputData.hook}`);
        console.log(JSON.stringify({ result: inputData.args }));
    }
} catch (error) {
    console.error('Error executing plugin:', error);
    console.log(JSON.stringify({ result: {} }));
}
"#;

        let runner_file = temp_dir.join("plugin_runner.js");
        let mut file = fs::File::create(&runner_file)?;
        file.write_all(runner_script.as_bytes())?;

        // 执行 Node.js 脚本，传入输入文件路径和插件路径作为参数
        let output = Command::new("node")
            .arg(&runner_file)
            .arg(&input_file)
            .arg(&plugin_path)
            .output()?;

        // 清理临时文件
        fs::remove_file(input_file)?;
        fs::remove_file(runner_file)?;

        // 解析输出
        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            println!("Plugin output: {}", stdout);

            // 尝试从输出中提取 JSON 结果
            let json_start = stdout.find('{');
            let json_end = stdout.rfind('}');

            if let (Some(start), Some(end)) = (json_start, json_end) {
                let json_str = &stdout[start..=end];

                if let Ok(json) = serde_json::from_str::<serde_json::Value>(json_str) {
                    if let Some(result) = json.get("result") {
                        return Ok(result.clone());
                    }
                }
            }

            // 如果没有找到有效的 JSON 输出，返回原始参数
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
