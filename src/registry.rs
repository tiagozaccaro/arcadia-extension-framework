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