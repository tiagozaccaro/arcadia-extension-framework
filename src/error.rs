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
#[cfg(test)]
mod tests {
    use super::*;
    use std::io;

    #[test]
    fn test_extension_error_from_io_error() {
        let io_err = io::Error::new(io::ErrorKind::NotFound, "file not found");
        let ext_err: ExtensionError = io_err.into();
        match ext_err {
            ExtensionError::Io(_) => {},
            _ => panic!("Expected Io error"),
        }
    }

    #[test]
    fn test_extension_error_from_serde_json_error() {
        let json_err = serde_json::from_str::<serde_json::Value>("invalid").unwrap_err();
        let ext_err: ExtensionError = json_err.into();
        match ext_err {
            ExtensionError::Json(_) => {},
            _ => panic!("Expected Json error"),
        }
    }

    #[test]
    fn test_extension_error_from_rusqlite_error() {
        let db_err = rusqlite::Error::QueryReturnedNoRows;
        let ext_err: ExtensionError = db_err.into();
        match ext_err {
            ExtensionError::Database(_) => {},
            _ => panic!("Expected Database error"),
        }
    }

    #[test]
    fn test_extension_error_display() {
        let err = ExtensionError::Validation("test message".to_string());
        assert_eq!(format!("{}", err), "Validation error: test message");
    }
}