use napi_derive::napi;
use std::path::Path;
use anyhow::Result;

// 重新导出loader_runner中的函数
pub use crate::loader_runner::{find_matching_loaders, apply_loaders};

#[napi(object)]
#[derive(Debug, Clone)]
pub struct Loader {
    pub path: String,
}

impl Loader {
    pub fn new(path: String) -> Self {
        Self { path }
    }
}
