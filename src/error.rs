#[derive(Debug)]
pub enum ExtensionError {
    Io(std::io::Error),
    Json(serde_json::Error),
    Database(rusqlite::Error),
    Validation(String),
    NotFound(String),
    PermissionDenied(String),
}

impl std::fmt::Display for ExtensionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExtensionError::Io(e) => write!(f, "IO error: {}", e),
            ExtensionError::Json(e) => write!(f, "JSON error: {}", e),
            ExtensionError::Database(e) => write!(f, "Database error: {}", e),
            ExtensionError::Validation(msg) => write!(f, "Validation error: {}", msg),
            ExtensionError::NotFound(msg) => write!(f, "Not found: {}", msg),
            ExtensionError::PermissionDenied(msg) => write!(f, "Permission denied: {}", msg),
        }
    }
}

impl std::error::Error for ExtensionError {}

impl From<std::io::Error> for ExtensionError {
    fn from(e: std::io::Error) -> Self {
        ExtensionError::Io(e)
    }
}

impl From<serde_json::Error> for ExtensionError {
    fn from(e: serde_json::Error) -> Self {
        ExtensionError::Json(e)
    }
}

impl From<rusqlite::Error> for ExtensionError {
    fn from(e: rusqlite::Error) -> Self {
        ExtensionError::Database(e)
    }
}