pub mod base;
pub mod release_v9;
pub mod desupport;

use serde_json::Value;
use crate::error::AppError;
use base::{Blob, XmlBlob};
use release_v9::ReleaseBlobV9;
use desupport::DesupportBlob;

pub enum BlobType {
    ReleaseV9(ReleaseBlobV9),
    Desupport(DesupportBlob),
}

impl BlobType {
    pub fn as_blob(&self) -> &dyn Blob {
        match self {
            BlobType::ReleaseV9(b) => b as &dyn Blob,
            BlobType::Desupport(b) => b as &dyn Blob,
        }
    }

    pub fn as_xml_blob(&self) -> Option<&dyn XmlBlob> {
        match self {
            BlobType::ReleaseV9(b) => Some(b as &dyn XmlBlob),
            BlobType::Desupport(b) => Some(b as &dyn XmlBlob),
        }
    }
}

/// Creates a blob from JSON data based on schema_version
pub fn create_blob(data: Value) -> Result<BlobType, AppError> {
    let schema_version = data.get("schema_version")
        .and_then(|v| v.as_i64())
        .unwrap_or(1) as i32;

    match schema_version {
        9 => Ok(BlobType::ReleaseV9(ReleaseBlobV9::new(data))),
        50 => Ok(BlobType::Desupport(DesupportBlob::new(data))),
        _ => Err(AppError::InvalidInput(format!("Unsupported schema version: {}", schema_version))),
    }
}
