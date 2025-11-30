use async_trait::async_trait;
use serde_json::Value;
use crate::models::{ExtensionManifest, ExtensionType};

#[async_trait]
pub trait ExtensionImpl: Send + Sync {
    async fn initialize(&mut self, context: &ExtensionContext) -> Result<(), ExtensionError>;
    async fn shutdown(&mut self) -> Result<(), ExtensionError>;
    async fn handle_hook(&self, hook: &str, params: Value) -> Result<Value, ExtensionError>;
    fn get_manifest(&self) -> &ExtensionManifest;
    fn get_type(&self) -> ExtensionType;
    fn get_id(&self) -> &str;
}

use std::path::PathBuf;
use tauri::AppHandle;
use crate::error::ExtensionError;

pub struct ExtensionContext {
    pub app_handle: AppHandle,
    pub extension_dir: PathBuf,
}