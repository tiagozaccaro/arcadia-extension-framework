use reqwest::Client;
use urlencoding;
use crate::models::ExtensionManifest;
use crate::store::models::{StoreExtension, StoreExtensionDetails, StoreFilters, SortOption};
use crate::store::error::StoreError;

pub struct ExtensionStoreClient {
    client: Client,
}

impl ExtensionStoreClient {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    pub async fn fetch_extensions(&self, base_url: &str, filters: &StoreFilters, sort: &SortOption, page: u32, limit: u32) -> Result<Vec<StoreExtension>, StoreError> {
        let mut url = format!("{}/extensions?page={}&limit={}", base_url, page, limit);

        if let Some(ext_type) = &filters.extension_type {
            url.push_str(&format!("&type={}", ext_type.to_string()));
        }
        if let Some(tags) = &filters.tags {
            url.push_str(&format!("&tags={}", tags.join(",")));
        }
        if let Some(search) = &filters.search {
            url.push_str(&format!("&search={}", urlencoding::encode(search)));
        }
        url.push_str(&format!("&sort={}", match sort {
            SortOption::Name => "name",
            SortOption::DownloadCount => "downloads",
            SortOption::Rating => "rating",
            SortOption::Newest => "newest",
        }));

        let response = self.client.get(&url).send().await?;
        let extensions: Vec<StoreExtension> = response.json().await?;
        Ok(extensions)
    }

    pub async fn fetch_extension_details(&self, base_url: &str, id: &str) -> Result<StoreExtensionDetails, StoreError> {
        let url = format!("{}/extensions/{}", base_url, id);
        let response = self.client.get(&url).send().await?;
        let details: StoreExtensionDetails = response.json().await?;
        Ok(details)
    }

    pub async fn download_manifest(&self, manifest_url: &str) -> Result<ExtensionManifest, StoreError> {
        let response = self.client.get(manifest_url).send().await?;
        let manifest: ExtensionManifest = response.json().await?;
        self.validate_manifest_security(&manifest)?;
        Ok(manifest)
    }

    pub async fn download_extension(&self, package_url: &str, checksum: &str) -> Result<Vec<u8>, StoreError> {
        let response = self.client.get(package_url).send().await?;
        let bytes = response.bytes().await?;
        let data = bytes.to_vec();

        // Validate checksum
        let computed_checksum = format!("{:x}", md5::compute(&data));
        if computed_checksum != checksum {
            return Err(StoreError::Security("Checksum mismatch".to_string()));
        }

        Ok(data)
    }

    fn validate_manifest_security(&self, manifest: &ExtensionManifest) -> Result<(), StoreError> {
        // Basic security validations
        if manifest.name.contains("..") || manifest.name.contains("/") {
            return Err(StoreError::Security("Invalid extension name".to_string()));
        }
        if manifest.entry_point.contains("..") {
            return Err(StoreError::Security("Invalid entry point".to_string()));
        }
        // Check for dangerous permissions
        let dangerous_perms = ["filesystem", "native"];
        for perm in &manifest.permissions {
            if dangerous_perms.contains(&perm.as_str()) {
                return Err(StoreError::Security(format!("Dangerous permission requested: {}", perm)));
            }
        }
        Ok(())
    }
}