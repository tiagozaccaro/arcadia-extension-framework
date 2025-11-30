use std::path::Path;
use crate::models::ExtensionManifest;
use crate::error::ExtensionError;

pub fn parse_manifest(manifest_path: &Path) -> Result<ExtensionManifest, ExtensionError> {
    let content = std::fs::read_to_string(manifest_path)?;
    let manifest: ExtensionManifest = serde_json::from_str(&content)?;
    Ok(manifest)
}

pub fn validate_manifest(manifest: &ExtensionManifest) -> Result<(), ExtensionError> {
    if manifest.name.is_empty() {
        return Err(ExtensionError::Validation("Name is required".to_string()));
    }
    if manifest.version.is_empty() {
        return Err(ExtensionError::Validation("Version is required".to_string()));
    }
    if manifest.entry_point.is_empty() {
        return Err(ExtensionError::Validation("Entry point is required".to_string()));
    }

    // Validate permissions
    let valid_permissions = ["filesystem", "network", "database", "ui", "native"];
    for perm in &manifest.permissions {
        if !valid_permissions.contains(&perm.as_str()) {
            return Err(ExtensionError::Validation(format!("Invalid permission: {}", perm)));
        }
    }

    Ok(())
}