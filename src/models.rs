use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ExtensionType {
    Theme,
    DataSource,
    GameLibrary,
}

impl From<String> for ExtensionType {
    fn from(s: String) -> Self {
        match s.as_str() {
            "theme" => ExtensionType::Theme,
            "data_source" => ExtensionType::DataSource,
            "game_library" => ExtensionType::GameLibrary,
            _ => ExtensionType::Theme, // default
        }
    }
}

impl std::fmt::Display for ExtensionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            ExtensionType::Theme => "theme",
            ExtensionType::DataSource => "data_source",
            ExtensionType::GameLibrary => "game_library",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtensionManifest {
    pub name: String,
    pub version: String,
    pub author: Option<String>,
    pub description: Option<String>,
    #[serde(rename = "type")]
    pub extension_type: ExtensionType,
    pub entry_point: String,
    pub permissions: Vec<String>,
    pub dependencies: Option<HashMap<String, String>>,
    pub hooks: Option<Vec<String>>,
    pub apis: Option<ExtensionApis>,
    #[serde(rename = "menuItems")]
    pub menu_items: Option<Vec<MenuItem>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MenuItem {
    pub title: String,
    pub url: String,
    pub icon: Option<String>,
    pub items: Option<Vec<MenuSubItem>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MenuSubItem {
    pub title: String,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtensionApis {
    pub provided: Option<Vec<String>>,
    pub required: Option<Vec<String>>,
}

#[derive(Debug, Clone)]
pub struct Extension {
    pub id: String,
    pub manifest: ExtensionManifest,
    pub path: std::path::PathBuf,
    pub enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExtensionInfo {
    pub id: String,
    pub name: String,
    pub version: String,
    pub author: Option<String>,
    pub description: Option<String>,
    pub extension_type: String,
    pub enabled: bool,
}

#[derive(Debug, Clone)]
pub struct ExtensionPermission {
    pub extension_id: String,
    pub permission: String,
    pub granted: bool,
}

#[derive(Debug, Clone)]
pub struct ExtensionSetting {
    pub extension_id: String,
    pub key: String,
    pub value: Option<String>,
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extension_type_from_string() {
        assert_eq!(ExtensionType::from("theme".to_string()), ExtensionType::Theme);
        assert_eq!(ExtensionType::from("data_source".to_string()), ExtensionType::DataSource);
        assert_eq!(ExtensionType::from("game_library".to_string()), ExtensionType::GameLibrary);
        assert_eq!(ExtensionType::from("unknown".to_string()), ExtensionType::Theme); // default
    }

    #[test]
    fn test_extension_type_to_string() {
        assert_eq!(ExtensionType::Theme.to_string(), "theme");
        assert_eq!(ExtensionType::DataSource.to_string(), "data_source");
        assert_eq!(ExtensionType::GameLibrary.to_string(), "game_library");
    }
}