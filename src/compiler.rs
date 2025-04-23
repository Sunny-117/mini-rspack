use napi_derive::napi;
use napi::threadsafe_function::{ThreadsafeFunction, ThreadsafeFunctionCallMode};
use napi::{JsFunction, Result};

use crate::RspackOptions;
use crate::plugin::SyncHook;

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

    // 简化版实现 - 在实际实现中，我们会创建一个编译过程
    // 这里我们只是模拟一个成功的编译

    // Call the done hook
    compiler.hooks.done.call(None);

    // Create a simple stats object
    let stats = Stats {
        entries: vec!["entry1.js".to_string(), "entry2.js".to_string()],
        chunks: vec!["chunk1".to_string(), "chunk2".to_string()],
        modules: vec!["module1".to_string(), "module2".to_string()],
        files: vec!["entry1.js".to_string(), "entry2.js".to_string()],
        assets: vec!["entry1.js".to_string(), "entry2.js".to_string()],
    };

    // Call the callback with the stats
    callback.call(
        Ok(stats),
        ThreadsafeFunctionCallMode::Blocking,
    );

    Ok(())
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
