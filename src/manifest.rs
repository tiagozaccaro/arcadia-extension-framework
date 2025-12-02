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
#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::ExtensionType;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_parse_manifest_valid() {
        let manifest = r#"{
            "name": "test-extension",
            "version": "1.0.0",
            "author": "Test Author",
            "description": "A test extension",
            "type": "Theme",
            "entry_point": "main.js",
            "permissions": ["filesystem", "ui"]
        }"#;

        let mut temp_file = NamedTempFile::new().unwrap();
        write!(temp_file, "{}", manifest).unwrap();

        let parsed = parse_manifest(temp_file.path()).unwrap();
        assert_eq!(parsed.name, "test-extension");
        assert_eq!(parsed.version, "1.0.0");
        assert_eq!(parsed.extension_type, ExtensionType::Theme);
    }

    #[test]
    fn test_validate_manifest_valid() {
        let manifest = ExtensionManifest {
            name: "test".to_string(),
            version: "1.0.0".to_string(),
            author: None,
            description: None,
            extension_type: ExtensionType::Theme,
            entry_point: "main.js".to_string(),
            permissions: vec!["filesystem".to_string(), "ui".to_string()],
            dependencies: None,
            hooks: None,
            apis: None,
            menu_items: None,
        };

        assert!(validate_manifest(&manifest).is_ok());
    }

    #[test]
    fn test_validate_manifest_invalid_name() {
        let manifest = ExtensionManifest {
            name: "".to_string(),
            version: "1.0.0".to_string(),
            author: None,
            description: None,
            extension_type: ExtensionType::Theme,
            entry_point: "main.js".to_string(),
            permissions: vec![],
            dependencies: None,
            hooks: None,
            apis: None,
            menu_items: None,
        };

        assert!(validate_manifest(&manifest).is_err());
    }

    #[test]
    fn test_validate_manifest_invalid_permission() {
        let manifest = ExtensionManifest {
            name: "test".to_string(),
            version: "1.0.0".to_string(),
            author: None,
            description: None,
            extension_type: ExtensionType::Theme,
            entry_point: "main.js".to_string(),
            permissions: vec!["invalid_perm".to_string()],
            dependencies: None,
            hooks: None,
            apis: None,
            menu_items: None,
        };

        assert!(validate_manifest(&manifest).is_err());
    }
}