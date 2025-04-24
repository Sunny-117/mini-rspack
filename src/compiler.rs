use napi_derive::napi;
use napi::threadsafe_function::{ThreadsafeFunction, ThreadsafeFunctionCallMode};
use napi::{JsFunction, Result};


use crate::RspackOptions;
use crate::plugin::SyncHook;
use crate::compilation::{Compilation, CompilationHooks};
use crate::plugin::register_plugin;

#[napi(object)]
#[derive(Debug, Clone)]
pub struct CompilerHooks {
    pub run: SyncHook,
    pub emit: SyncHook,
    pub done: SyncHook,
}

#[napi(object)]
#[derive(Debug, Clone)]
pub struct Stats {
    pub entries: Vec<String>,
    pub chunks: Vec<String>,
    pub modules: Vec<String>,
    pub files: Vec<String>,
    pub assets: Vec<String>,
}

#[napi]
#[derive(Debug, Clone)]
pub struct Compiler {
    pub options: RspackOptions,
    pub hooks: CompilerHooks,
}

#[napi]
impl Compiler {
    #[napi]
    pub fn run(&self, callback: JsFunction) -> Result<()> {
        // 将JsFunction转换为ThreadsafeFunction
        let tsfn: ThreadsafeFunction<Stats> = callback.create_threadsafe_function(0, |ctx| {
            Ok(vec![ctx.value])
        })?;

        // 调用内部实现
        run_compiler_internal(self, tsfn)
    }

    #[napi]
    pub fn watch(&self, callback: JsFunction) -> Result<()> {
        // 将JsFunction转换为ThreadsafeFunction
        let tsfn: ThreadsafeFunction<Stats> = callback.create_threadsafe_function(0, |ctx| {
            Ok(vec![ctx.value])
        })?;

        // 调用内部实现
        watch_compiler_internal(self, tsfn)
    }
}

#[napi]
pub fn create_compiler(options: RspackOptions) -> Compiler {
    Compiler {
        options,
        hooks: CompilerHooks {
            run: SyncHook::new("run"),
            emit: SyncHook::new("emit"),
            done: SyncHook::new("done"),
        },
    }
}

// 内部函数，不导出到JS
fn run_compiler_internal(compiler: &Compiler, callback: ThreadsafeFunction<Stats>) -> Result<()> {
    // Call the run hook
    compiler.hooks.run.call(None);

    // 创建一个编译实例
    let compilation_hooks = CompilationHooks {
        emit: SyncHook::new("emit"),
    };

    let mut compilation = Compilation::new(compiler.options.clone(), compilation_hooks);

    // 注册插件
    if let Some(plugins) = &compiler.options.plugins {
        for plugin_name in plugins {
            register_plugin(&mut compilation, plugin_name);
        }
    }

    // 执行编译过程
    match compilation.make() {
        Ok(_) => {
            // 编译成功
            // Call the done hook
            compiler.hooks.done.call(None);

            // 创建真实的stats对象
            let entries = compilation.entries.iter().map(|chunk| chunk.name.clone()).collect();
            let chunks = compilation.chunks.iter().map(|chunk| chunk.name.clone()).collect();
            let modules = compilation.modules.iter().map(|module| module.id.clone()).collect();
            let files = compilation.files.clone();
            let assets = compilation.assets.keys().cloned().collect();

            let stats = Stats {
                entries,
                chunks,
                modules,
                files,
                assets,
            };

            // Call the callback with the stats
            callback.call(
                Ok(stats),
                ThreadsafeFunctionCallMode::Blocking,
            );

            Ok(())
        },
        Err(err) => {
            // 编译失败
            eprintln!("Compilation failed: {:?}", err);

            // 创建空的stats对象
            let _stats = Stats {
                entries: vec![],
                chunks: vec![],
                modules: vec![],
                files: vec![],
                assets: vec![],
            };

            // Call the callback with an error
            callback.call(
                Err(napi::Error::new(napi::Status::GenericFailure, format!("Compilation failed: {:?}", err))),
                ThreadsafeFunctionCallMode::Blocking,
            );

            Ok(())
        }
    }
}

// 内部函数，不导出到JS
fn watch_compiler_internal(compiler: &Compiler, callback: ThreadsafeFunction<Stats>) -> Result<()> {
    // First run
    run_compiler_internal(compiler, callback.clone())?;

    // If watch is enabled, set up file watchers
    if let Some(true) = compiler.options.watch {
        // In a real implementation, we would set up file watchers here
        // For now, we'll just log that watch mode is enabled
        println!("Watch mode enabled");
    }

    Ok(())
}

// 这些函数保留但不再导出到JS
#[napi]
pub fn run_compiler(compiler: &Compiler, callback: ThreadsafeFunction<Stats>) -> Result<()> {
    run_compiler_internal(compiler, callback)
}

#[napi]
pub fn watch_compiler(compiler: &Compiler, callback: ThreadsafeFunction<Stats>) -> Result<()> {
    watch_compiler_internal(compiler, callback)
}
