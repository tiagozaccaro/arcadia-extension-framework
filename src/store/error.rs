#[derive(Debug)]
pub enum StoreError {
    Network(reqwest::Error),
    Json(serde_json::Error),
    Validation(String),
    Security(String),
}

impl std::fmt::Display for StoreError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StoreError::Network(e) => write!(f, "Network error: {}", e),
            StoreError::Json(e) => write!(f, "JSON error: {}", e),
            StoreError::Validation(msg) => write!(f, "Validation error: {}", msg),
            StoreError::Security(msg) => write!(f, "Security error: {}", msg),
        }
    }
}

impl std::error::Error for StoreError {}

impl From<reqwest::Error> for StoreError {
    fn from(e: reqwest::Error) -> Self {
        StoreError::Network(e)
    }
}

impl From<serde_json::Error> for StoreError {
    fn from(e: serde_json::Error) -> Self {
        StoreError::Json(e)
    }
}