use napi_derive::napi;

use crate::WebpackOptions;
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
    pub options: WebpackOptions,
    pub hooks: CompilerHooks,
}

#[napi]
pub fn create_compiler(options: WebpackOptions) -> Compiler {
    Compiler {
        options,
        hooks: CompilerHooks {
            run: SyncHook::new("run"),
            emit: SyncHook::new("emit"),
            done: SyncHook::new("done"),
        },
    }
}

#[napi]
pub fn run_compiler(compiler: Compiler, callback: napi::threadsafe_function::ThreadsafeFunction<Stats>) -> napi::Result<()> {
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
        napi::threadsafe_function::ThreadsafeFunctionCallMode::Blocking,
    );

    Ok(())
}

#[napi]
pub fn watch_compiler(compiler: Compiler, callback: napi::threadsafe_function::ThreadsafeFunction<Stats>) -> napi::Result<()> {
    // First run
    run_compiler(compiler.clone(), callback.clone())?;

    // If watch is enabled, set up file watchers
    if let Some(true) = compiler.options.watch {
        // In a real implementation, we would set up file watchers here
        // For now, we'll just log that watch mode is enabled
        println!("Watch mode enabled");
    }

    Ok(())
}
