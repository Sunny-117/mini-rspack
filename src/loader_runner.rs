use std::path::Path;
use anyhow::Result;
use std::process::Command;
use std::fs;
use std::io::Write;
use std::env;

// Loader上下文，包含当前加载的模块信息
pub struct LoaderContext {
    pub resource_path: String,
    pub resource_query: Option<String>,
    pub resource_fragment: Option<String>,
    pub context_directory: String,
    pub target: String,
    pub options: serde_json::Value,
}

// Loader Runner，负责执行一系列loader
pub struct LoaderRunner {
    pub loaders: Vec<String>,
    pub resource: String,
    pub context: LoaderContext,
}

impl LoaderRunner {
    // 创建一个新的Loader Runner
    pub fn new(loaders: Vec<String>, resource: String, context_directory: String) -> Self {
        // 解析资源路径、查询参数和片段
        let (resource_path, resource_query, resource_fragment) = parse_resource(&resource);

        // 创建上下文
        let context = LoaderContext {
            resource_path: resource_path.to_string(),
            resource_query,
            resource_fragment,
            context_directory,
            target: "web".to_string(),
            options: serde_json::json!({}),
        };

        Self {
            loaders,
            resource,
            context,
        }
    }

    // 运行所有loader
    pub fn run(&self, source_code: &str) -> Result<String> {
        let mut processed_code = source_code.to_string();

        // 按照从右到左的顺序执行loader
        for loader_path in self.loaders.iter().rev() {
            processed_code = self.run_loader(loader_path, &processed_code)?;
        }

        Ok(processed_code)
    }

    // 运行单个loader
    fn run_loader(&self, loader_path: &str, source_code: &str) -> Result<String> {
        println!("Running loader: {} on resource: {}", loader_path, self.resource);

        // 检查loader文件是否存在
        let loader_full_path = if loader_path.starts_with("./") || loader_path.starts_with("../") {
            // 相对路径
            let base_dir = std::env::current_dir().unwrap_or_else(|_| Path::new(".").to_path_buf());
            base_dir.join(loader_path)
        } else {
            // 绝对路径或node_modules中的loader
            Path::new(loader_path).to_path_buf()
        };

        if !loader_full_path.exists() {
            return Err(anyhow::anyhow!("Loader not found: {}", loader_path));
        }

        // 准备loader的输入
        let loader_input = serde_json::json!({
            "source": source_code,
            "resourcePath": self.context.resource_path,
            "resourceQuery": self.context.resource_query,
            "resourceFragment": self.context.resource_fragment,
            "context": self.context.context_directory,
            "target": self.context.target,
            "options": self.context.options,
        });

        // 创建临时文件来存储loader的输入
        let temp_dir = env::temp_dir();
        let input_file = temp_dir.join("loader_input.json");
        let mut file = fs::File::create(&input_file)?;
        file.write_all(loader_input.to_string().as_bytes())?;

        // 创建一个Node.js脚本来执行loader
        let runner_script = format!(
            r#"
            const fs = require('fs');
            const path = require('path');

            // 读取输入
            const inputData = JSON.parse(fs.readFileSync('{}'));

            // 加载loader
            const loader = require('{}');

            // 创建上下文
            const loaderContext = {{
                resourcePath: inputData.resourcePath,
                resourceQuery: inputData.resourceQuery,
                resourceFragment: inputData.resourceFragment,
                context: inputData.context,
                target: inputData.target,
                options: inputData.options,
                async: function() {{
                    const callback = this;
                    return function(err, result) {{
                        if (err) {{
                            console.error(err);
                            process.exit(1);
                        }}
                        console.log(JSON.stringify({{ result }}));
                    }};
                }},
            }};

            // 执行loader
            const result = loader.call(loaderContext, inputData.source);

            // 输出结果
            if (result !== undefined) {{
                console.log(JSON.stringify({{ result }}));
            }}
            "#,
            input_file.to_string_lossy(),
            loader_full_path.to_string_lossy()
        );

        let runner_file = temp_dir.join("loader_runner.js");
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

            // 查找最后一个JSON对象（忽略之前的控制台输出）
            let json_start = stdout.rfind("{").unwrap_or(0);
            let json_str = &stdout[json_start..];

            // 解析JSON输出
            if let Ok(json) = serde_json::from_str::<serde_json::Value>(json_str) {
                if let Some(result) = json.get("result") {
                    if let Some(result_str) = result.as_str() {
                        // 递归解析JSON结果
                        fn extract_result(source: &str) -> String {
                            if source.starts_with("{") && source.contains("\"result\":") {
                                if let Ok(json) = serde_json::from_str::<serde_json::Value>(source) {
                                    if let Some(result) = json.get("result") {
                                        if let Some(result_str) = result.as_str() {
                                            return extract_result(result_str);
                                        }
                                    }
                                }
                            }
                            source.to_string()
                        }

                        return Ok(extract_result(result_str));
                    }
                }
            }

            // 如果没有JSON输出，返回原始源代码
            Ok(source_code.to_string())
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            Err(anyhow::anyhow!("Loader execution failed: {}", stderr))
        }
    }
}

// 解析资源路径，分离查询参数和片段
fn parse_resource(resource: &str) -> (&str, Option<String>, Option<String>) {
    // 分离片段
    let parts: Vec<&str> = resource.split('#').collect();
    let (resource_without_fragment, fragment) = match parts.len() {
        1 => (parts[0], None),
        _ => (parts[0], Some(parts[1..].join("#"))),
    };

    // 分离查询参数
    let parts: Vec<&str> = resource_without_fragment.split('?').collect();
    let (path, query) = match parts.len() {
        1 => (parts[0], None),
        _ => (parts[0], Some(parts[1..].join("?"))),
    };

    (path, query, fragment)
}

// 查找匹配的loaders
pub fn find_matching_loaders(
    module_path: &Path,
    rules: &[crate::RuleOptions],
) -> Vec<crate::loader::Loader> {
    let mut loaders = Vec::new();

    for rule in rules {
        // 获取模块路径的字符串表示
        let path_str = module_path.to_string_lossy().to_string();

        // 检查模块是否匹配规则
        let is_match = match_rule(&path_str, &rule.test);

        if is_match {
            // 添加规则中的所有loader
            for loader_path in &rule.use_ {
                loaders.push(crate::loader::Loader { path: loader_path.clone() });
            }
        }
    }

    loaders
}

// 匹配规则
fn match_rule(path: &str, test: &str) -> bool {
    // 简单的字符串匹配（在实际实现中，这会使用正则表达式）
    path.ends_with(test)
}

// 应用loaders
pub fn apply_loaders(
    source_code: &str,
    loaders: &[crate::loader::Loader],
    _name: &str,
    module_path: &str,
) -> Result<String> {
    let processed_code = source_code.to_string();

    // 如果没有loader，直接返回源代码
    if loaders.is_empty() {
        return Ok(processed_code);
    }

    // 获取loader路径
    let loader_paths: Vec<String> = loaders.iter().map(|l| l.path.clone()).collect();

    // 获取上下文目录（从module_path中提取）
    let context_directory = std::path::Path::new(module_path)
        .parent()
        .unwrap_or_else(|| std::path::Path::new("."))
        .to_string_lossy()
        .to_string();

    // 创建Loader Runner
    let runner = LoaderRunner::new(
        loader_paths,
        module_path.to_string(),
        context_directory,
    );

    // 运行loaders
    runner.run(&processed_code)
}
