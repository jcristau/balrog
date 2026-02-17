use serde_json::Value;
use crate::blobs::base::{Blob, XmlBlob, ServeUpdate};
use crate::update::query::UpdateQuery;
use crate::error::AppError;

pub struct ReleaseBlobV9 {
    data: Value,
}

impl ReleaseBlobV9 {
    pub fn new(data: Value) -> Self {
        ReleaseBlobV9 { data }
    }

    /// Gets URL template from fileUrls section with channel and from fallbacks
    fn get_url_template(&self, channel: &str, patch_type: &str, from: &str) -> Option<&str> {
        let file_urls = self.data.get("fileUrls")?;

        // Try exact channel match first, then wildcard
        for ch in &[channel, "*"] {
            if let Some(channel_urls) = file_urls.get(ch) {
                if let Some(type_urls) = channel_urls.get(patch_type) {
                    // Try exact from match first, then wildcard
                    for f in &[from, "*"] {
                        if let Some(url) = type_urls.get(f).and_then(|v| v.as_str()) {
                            return Some(url);
                        }
                    }
                }
            }
        }

        None
    }

    /// Resolves platform aliases (e.g., WINNT_x86_64-msvc-x64 -> WINNT_x86_64-msvc)
    fn get_platform_data(&self, build_target: &str) -> Option<&Value> {
        let platforms = self.data.get("platforms")?;

        // Try exact match first
        if let Some(platform) = platforms.get(build_target) {
            return Some(platform);
        }

        // Try alias resolution
        if let Some(platform) = platforms.as_object()?.values().find(|p| {
            if let Some(aliases) = p.get("alias").and_then(|a| a.as_array()) {
                aliases.iter().any(|a| a.as_str() == Some(build_target))
            } else if let Some(alias) = p.get("alias").and_then(|a| a.as_str()) {
                alias == build_target
            } else {
                false
            }
        }) {
            return Some(platform);
        }

        None
    }

    /// Gets locale data for the platform
    fn get_locale_data<'a>(&'a self, platform: &'a Value, locale: &str) -> Option<&'a Value> {
        let locales = platform.get("locales")?;

        // Try exact match
        if let Some(loc) = locales.get(locale) {
            return Some(loc);
        }

        // Try partial locale (en-US -> en)
        if let Some(short_locale) = locale.split('-').next() {
            if let Some(loc) = locales.get(short_locale) {
                return Some(loc);
            }
        }

        None
    }

    /// Builds update URL with substitutions
    fn build_url(&self, url_template: &str, query: &UpdateQuery, locale: &str) -> Result<String, AppError> {
        let mut url = url_template.to_string();

        // Perform substitutions
        url = url.replace("%LOCALE%", locale);
        url = url.replace("%OS_FTP%", &self.get_os_ftp(&query.build_target));
        url = url.replace("%OS_BOUNCER%", &self.get_os_bouncer(&query.build_target));

        // Validate URL
        if self.is_forbidden_url(&url) {
            return Err(AppError::InvalidInput("Forbidden URL".to_string()));
        }

        Ok(url)
    }

    fn get_os_ftp(&self, build_target: &str) -> String {
        if build_target.starts_with("WINNT") {
            "win64".to_string()
        } else if build_target.starts_with("Darwin") {
            "mac".to_string()
        } else if build_target.starts_with("Linux") {
            "linux-x86_64".to_string()
        } else {
            build_target.to_string()
        }
    }

    fn get_os_bouncer(&self, build_target: &str) -> String {
        if build_target.starts_with("WINNT") {
            "win64".to_string()
        } else if build_target.starts_with("Darwin") {
            "osx".to_string()
        } else if build_target.starts_with("Linux") {
            "linux64".to_string()
        } else {
            build_target.to_string()
        }
    }

    fn is_forbidden_url(&self, url: &str) -> bool {
        // For PoC: Allow common Mozilla/Firefox CDN domains
        // Production should use proper allowlist configuration
        let allowed_domains = vec![
            "download.mozilla.org",
            "archive.mozilla.org",
            "download-installer.cdn.mozilla.net",
            "mozilla-nightly-updates.s3.amazonaws.com",
            "cdn.mozilla.net",
            "ftp.mozilla.org",
            "releases.mozilla.org",
            "updates.cdn.mozilla.net",
            "aus5.mozilla.org",
        ];

        // Extract domain from URL
        if let Some(domain_start) = url.find("://") {
            let rest = &url[domain_start + 3..];
            if let Some(domain_end) = rest.find('/') {
                let domain = &rest[..domain_end];

                // Check if domain is in allowed list or subdomain of allowed
                for allowed in &allowed_domains {
                    if domain == *allowed || domain.ends_with(&format!(".{}", allowed)) {
                        return false;
                    }
                }

                tracing::warn!("URL blocked - domain not in allowlist: {}", domain);
                return true;
            }
        }

        // If we can't parse the domain, be conservative
        tracing::warn!("URL blocked - failed to parse: {}", url);
        true
    }
}

impl Blob for ReleaseBlobV9 {
    fn schema_version(&self) -> i32 {
        9
    }

    fn should_serve_update(&self, query: &UpdateQuery) -> Result<ServeUpdate, AppError> {
        // Get blob version and buildID
        let blob_version = self.data.get("appVersion")
            .and_then(|v| v.as_str())
            .or_else(|| self.data.get("version").and_then(|v| v.as_str()))
            .unwrap_or("");

        let blob_build_id = self.data.get("platformVersion")
            .and_then(|v| v.as_str())
            .unwrap_or("");

        // Parse versions for comparison
        use crate::rule_matching::version::MozillaVersion;

        let query_ver = MozillaVersion::parse(&query.version)
            .map_err(|e| AppError::InvalidInput(e))?;

        let blob_ver = MozillaVersion::parse(blob_version)
            .map_err(|e| AppError::InvalidInput(e))?;

        // If blob version is less than query version, don't serve
        if blob_ver < query_ver {
            return Ok(ServeUpdate::No);
        }

        // If versions are equal, check buildID
        if blob_ver == query_ver {
            if blob_build_id <= query.build_id.as_str() {
                return Ok(ServeUpdate::No);
            }
        }

        // If force is set, serve immediately
        if query.force.is_some() {
            return Ok(ServeUpdate::Yes);
        }

        // Otherwise, might need pinnable release check
        Ok(ServeUpdate::Maybe)
    }
}

impl XmlBlob for ReleaseBlobV9 {
    fn get_inner_header_xml(&self, _query: &UpdateQuery) -> Result<String, AppError> {
        let update_type = self.data.get("updateType")
            .and_then(|v| v.as_str())
            .unwrap_or("minor");

        let display_version = self.data.get("displayVersion")
            .and_then(|v| v.as_str())
            .unwrap_or("");

        let app_version = self.data.get("appVersion")
            .and_then(|v| v.as_str())
            .unwrap_or("");

        let platform_version = self.data.get("platformVersion")
            .and_then(|v| v.as_str())
            .unwrap_or("");

        let build_id = self.data.get("buildID")
            .and_then(|v| v.as_str())
            .unwrap_or("");

        let details_url = self.data.get("detailsUrl")
            .and_then(|v| v.as_str())
            .unwrap_or("");

        let actions = self.data.get("actions")
            .and_then(|v| v.as_str())
            .unwrap_or("showURL");

        let mut parts = vec![
            format!(r#"    <update type="{}""#, update_type),
        ];

        if !display_version.is_empty() {
            parts.push(format!(r#" displayVersion="{}""#, display_version));
        }

        if !app_version.is_empty() {
            parts.push(format!(r#" appVersion="{}""#, app_version));
        }

        if !platform_version.is_empty() {
            parts.push(format!(r#" platformVersion="{}""#, platform_version));
        }

        if !build_id.is_empty() {
            parts.push(format!(r#" buildID="{}""#, build_id));
        }

        if !details_url.is_empty() {
            parts.push(format!(r#" detailsURL="{}""#, details_url));
        }

        if !actions.is_empty() {
            parts.push(format!(r#" actions="{}""#, actions));
        }

        parts.push(">".to_string());

        Ok(parts.join(""))
    }

    fn get_inner_xml(&self, query: &UpdateQuery) -> Result<String, AppError> {
        let platform = self.get_platform_data(&query.build_target)
            .ok_or_else(|| {
                tracing::warn!("Platform not found: {}. Available platforms: {:?}",
                    query.build_target,
                    self.data.get("platforms").and_then(|p| p.as_object()).map(|o| o.keys().collect::<Vec<_>>()));
                AppError::NotFound(format!("Platform not found: {}", query.build_target))
            })?;

        let locale_data = self.get_locale_data(platform, &query.locale)
            .ok_or_else(|| {
                tracing::warn!("Locale not found: {}. Available locales: {:?}",
                    query.locale,
                    platform.get("locales").and_then(|l| l.as_object()).map(|o| o.keys().collect::<Vec<_>>()));
                AppError::NotFound(format!("Locale not found: {}", query.locale))
            })?;

        let mut patches = Vec::new();

        // Handle complete patches
        if let Some(completes) = locale_data.get("completes").and_then(|v| v.as_array()) {
            for complete in completes {
                let from = complete.get("from").and_then(|v| v.as_str()).unwrap_or("*");
                let url_template = self.get_url_template(&query.channel, "completes", from)
                    .ok_or_else(|| AppError::NotFound(format!("No URL template for complete patch from {}", from)))?;

                let hash_function = complete.get("hashFunction")
                    .and_then(|v| v.as_str())
                    .unwrap_or("sha512");
                let hash_value = complete.get("hashValue")
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                let size = complete.get("filesize")
                    .and_then(|v| v.as_u64())
                    .unwrap_or(0);

                let built_url = self.build_url(url_template, query, &query.locale)?;

                patches.push(format!(
                    r#"        <patch type="complete" URL="{}" size="{}" {}="{}"/>"#,
                    built_url, size, hash_function, hash_value
                ));
            }
        }

        // Handle partial patches
        if let Some(partials) = locale_data.get("partials").and_then(|v| v.as_array()) {
            for partial in partials {
                let from = partial.get("from").and_then(|v| v.as_str()).unwrap_or("*");
                let url_template = self.get_url_template(&query.channel, "partials", from)
                    .ok_or_else(|| AppError::NotFound(format!("No URL template for partial patch from {}", from)))?;

                let hash_function = partial.get("hashFunction")
                    .and_then(|v| v.as_str())
                    .unwrap_or("sha512");
                let hash_value = partial.get("hashValue")
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                let size = partial.get("filesize")
                    .and_then(|v| v.as_u64())
                    .unwrap_or(0);

                let built_url = self.build_url(url_template, query, &query.locale)?;

                patches.push(format!(
                    r#"        <patch type="partial" URL="{}" size="{}" {}="{}"/>"#,
                    built_url, size, hash_function, hash_value
                ));
            }
        }

        Ok(patches.join("\n"))
    }

    fn get_inner_footer_xml(&self) -> String {
        "    </update>".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_get_os_bouncer() {
        let blob = ReleaseBlobV9::new(json!({}));

        assert_eq!(blob.get_os_bouncer("WINNT_x86_64-msvc"), "win64");
        assert_eq!(blob.get_os_bouncer("Darwin_x86_64-gcc3"), "osx");
        assert_eq!(blob.get_os_bouncer("Linux_x86_64-gcc3"), "linux64");
    }
}
