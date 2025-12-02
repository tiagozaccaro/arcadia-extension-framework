use std::collections::HashMap;
use crate::models::ExtensionInfo;

pub struct ExtensionRegistry {
    extensions: HashMap<String, ExtensionInfo>,
}

impl ExtensionRegistry {
    pub fn new() -> Self {
        Self {
            extensions: HashMap::new(),
        }
    }

    pub fn register(&mut self, extension: ExtensionInfo) {
        self.extensions.insert(extension.id.clone(), extension);
    }

    pub fn unregister(&mut self, id: &str) {
        self.extensions.remove(id);
    }

    pub fn get(&self, id: &str) -> Option<&ExtensionInfo> {
        self.extensions.get(id)
    }

    pub fn get_all(&self) -> Vec<ExtensionInfo> {
        self.extensions.values().cloned().collect()
    }

    pub fn get_enabled(&self) -> Vec<ExtensionInfo> {
        self.extensions.values().filter(|e| e.enabled).cloned().collect()
    }

    pub fn get_mut(&mut self, id: &str) -> Option<&mut ExtensionInfo> {
        self.extensions.get_mut(id)
    }
}

impl Default for ExtensionRegistry {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_registry_new() {
        let registry = ExtensionRegistry::new();
        assert!(registry.get_all().is_empty());
    }

    #[test]
    fn test_registry_register_and_get() {
        let mut registry = ExtensionRegistry::new();
        let ext = ExtensionInfo {
            id: "test".to_string(),
            name: "Test Extension".to_string(),
            version: "1.0.0".to_string(),
            author: Some("Author".to_string()),
            description: Some("Description".to_string()),
            extension_type: "theme".to_string(),
            enabled: true,
        };

        registry.register(ext.clone());
        assert_eq!(registry.get("test"), Some(&ext));
    }

    #[test]
    fn test_registry_unregister() {
        let mut registry = ExtensionRegistry::new();
        let ext = ExtensionInfo {
            id: "test".to_string(),
            name: "Test Extension".to_string(),
            version: "1.0.0".to_string(),
            author: None,
            description: None,
            extension_type: "theme".to_string(),
            enabled: true,
        };

        registry.register(ext);
        assert!(registry.get("test").is_some());
        registry.unregister("test");
        assert!(registry.get("test").is_none());
    }

    #[test]
    fn test_registry_get_all() {
        let mut registry = ExtensionRegistry::new();
        let ext1 = ExtensionInfo {
            id: "test1".to_string(),
            name: "Test 1".to_string(),
            version: "1.0.0".to_string(),
            author: None,
            description: None,
            extension_type: "theme".to_string(),
            enabled: true,
        };
        let ext2 = ExtensionInfo {
            id: "test2".to_string(),
            name: "Test 2".to_string(),
            version: "1.0.0".to_string(),
            author: None,
            description: None,
            extension_type: "theme".to_string(),
            enabled: false,
        };

        registry.register(ext1.clone());
        registry.register(ext2.clone());

        let all = registry.get_all();
        assert_eq!(all.len(), 2);
    }

    #[test]
    fn test_registry_get_enabled() {
        let mut registry = ExtensionRegistry::new();
        let ext1 = ExtensionInfo {
            id: "test1".to_string(),
            name: "Test 1".to_string(),
            version: "1.0.0".to_string(),
            author: None,
            description: None,
            extension_type: "theme".to_string(),
            enabled: true,
        };
        let ext2 = ExtensionInfo {
            id: "test2".to_string(),
            name: "Test 2".to_string(),
            version: "1.0.0".to_string(),
            author: None,
            description: None,
            extension_type: "theme".to_string(),
            enabled: false,
        };

        registry.register(ext1.clone());
        registry.register(ext2);

        let enabled = registry.get_enabled();
        assert_eq!(enabled.len(), 1);
        assert_eq!(enabled[0].id, "test1");
    }
}