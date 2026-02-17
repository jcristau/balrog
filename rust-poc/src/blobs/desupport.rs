use serde_json::Value;
use crate::blobs::base::{Blob, XmlBlob, ServeUpdate};
use crate::update::query::UpdateQuery;
use crate::error::AppError;

pub struct DesupportBlob {
    data: Value,
}

impl DesupportBlob {
    pub fn new(data: Value) -> Self {
        DesupportBlob { data }
    }
}

impl Blob for DesupportBlob {
    fn schema_version(&self) -> i32 {
        50
    }

    fn should_serve_update(&self, _query: &UpdateQuery) -> Result<ServeUpdate, AppError> {
        // Desupport blobs always serve updates
        Ok(ServeUpdate::Yes)
    }
}

impl XmlBlob for DesupportBlob {
    fn get_inner_header_xml(&self, _query: &UpdateQuery) -> Result<String, AppError> {
        let details_url = self.data["detailsUrl"].as_str().unwrap_or("");

        Ok(format!(
            r#"    <update type="minor" unsupported="true" detailsURL="{}">"#,
            details_url
        ))
    }

    fn get_inner_xml(&self, _query: &UpdateQuery) -> Result<String, AppError> {
        Ok(String::new())
    }
}
