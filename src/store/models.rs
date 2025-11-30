use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::models::ExtensionType;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum StoreSourceType {
    Official,
    Community,
    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoreSource {
    pub id: String,
    pub name: String,
    pub source_type: StoreSourceType,
    pub base_url: String,
    pub enabled: bool,
    pub priority: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoreExtension {
    pub id: String,
    pub name: String,
    pub version: String,
    pub author: String,
    pub description: String,
    pub extension_type: ExtensionType,
    pub download_count: u32,
    pub rating: f32,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoreExtensionDetails {
    pub id: String,
    pub name: String,
    pub version: String,
    pub author: String,
    pub description: String,
    pub extension_type: ExtensionType,
    pub download_count: u32,
    pub rating: f32,
    pub tags: Vec<String>,
    pub manifest_url: String,
    pub package_url: String,
    pub checksum: String,
    pub readme: String,
    pub screenshots: Vec<String>,
    pub dependencies: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoreFilters {
    pub extension_type: Option<ExtensionType>,
    pub tags: Option<Vec<String>>,
    pub search: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SortOption {
    Name,
    DownloadCount,
    Rating,
    Newest,
}