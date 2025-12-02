use std::collections::HashMap;
use crate::store::models::{StoreSource, StoreSourceType};
use crate::store::error::StoreError;

pub struct StoreManager {
    sources: HashMap<String, StoreSource>,
}

impl StoreManager {
    pub fn new() -> Self {
        let mut sources = HashMap::new();

        // Add default local source
        let default_source = StoreSource {
            id: "default".to_string(),
            name: "Local Arcadia Store".to_string(),
            source_type: StoreSourceType::Official,
            base_url: "https://github.com/tiagozaccaro/arcadia-app/blob/main/arcadia-store/store-manifest.json".to_string(),
            enabled: true,
            priority: 0,
        };
        eprintln!("DEBUG: Default store source URL: {}", default_source.base_url);
        sources.insert(default_source.id.clone(), default_source);

        Self { sources }
    }

    pub fn add_source(&mut self, source: StoreSource) -> Result<(), StoreError> {
        if self.sources.contains_key(&source.id) {
            return Err(StoreError::Validation("Source with this ID already exists".to_string()));
        }
        self.validate_source(&source)?;
        self.sources.insert(source.id.clone(), source);
        Ok(())
    }

    pub fn remove_source(&mut self, id: &str) -> Result<(), StoreError> {
        if id == "default" {
            return Err(StoreError::Validation("Cannot remove default source".to_string()));
        }
        self.sources.remove(id);
        Ok(())
    }

    pub fn update_source(&mut self, source: StoreSource) -> Result<(), StoreError> {
        if !self.sources.contains_key(&source.id) {
            return Err(StoreError::Validation("Source not found".to_string()));
        }
        if source.id == "default" && source.source_type != StoreSourceType::Official {
            return Err(StoreError::Validation("Cannot change type of default source".to_string()));
        }
        self.validate_source(&source)?;
        self.sources.insert(source.id.clone(), source);
        Ok(())
    }

    pub fn get_source(&self, id: &str) -> Option<&StoreSource> {
        self.sources.get(id)
    }

    pub fn list_sources(&self) -> Vec<StoreSource> {
        let mut sources: Vec<_> = self.sources.values().cloned().collect();
        sources.sort_by_key(|s| s.priority);
        sources
    }

    pub fn get_enabled_sources(&self) -> Vec<StoreSource> {
        self.sources.values().filter(|s| s.enabled).cloned().collect()
    }

    fn validate_source(&self, source: &StoreSource) -> Result<(), StoreError> {
        if source.name.trim().is_empty() {
            return Err(StoreError::Validation("Source name cannot be empty".to_string()));
        }
        if source.base_url.trim().is_empty() {
            if !matches!(source.source_type, StoreSourceType::Official) {
                return Err(StoreError::Validation("Base URL cannot be empty".to_string()));
            }
        } else {
            // Validate URL format
            if url::Url::parse(&source.base_url).is_err() {
                return Err(StoreError::Validation("Invalid URL format".to_string()));
            }

            // Security validations for custom sources
            if matches!(source.source_type, StoreSourceType::Custom) {
                self.validate_custom_url(&source.base_url)?;
            }
        }

        Ok(())
    }

    fn validate_custom_url(&self, url: &str) -> Result<(), StoreError> {
        // Only allow HTTPS for custom sources
        if !url.starts_with("https://") {
            return Err(StoreError::Security("Custom sources must use HTTPS".to_string()));
        }

        // Check for potentially dangerous URLs
        let blocked_domains = ["localhost", "127.0.0.1", "0.0.0.0", "10.0.0.0/8", "172.16.0.0/12", "192.168.0.0/16"];
        for domain in &blocked_domains {
            if url.contains(domain) {
                return Err(StoreError::Security(format!("Blocked domain: {}", domain)));
            }
        }

        Ok(())
    }
}