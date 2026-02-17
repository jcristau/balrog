use sqlx::FromRow;
use serde_json::Value;

#[derive(Debug, Clone, FromRow)]
pub struct RuleRow {
    pub rule_id: i32,
    pub priority: i32,
    pub mapping: Option<String>,
    pub fallback_mapping: Option<String>,
    pub background_rate: i32,
    pub product: Option<String>,
    pub version: Option<String>,
    pub build_id: Option<String>,
    pub channel: Option<String>,
    pub build_target: Option<String>,
    pub locale: Option<String>,
    pub os_version: Option<String>,
    pub instruction_set: Option<String>,
    pub memory: Option<i64>,
    pub mig64: Option<bool>,
    pub jaws: Option<bool>,
    pub distribution: Option<String>,
    pub dist_version: Option<String>,
    pub header_architecture: Option<String>,
    pub data_version: i32,
}

#[derive(Debug, Clone, FromRow)]
pub struct ReleasesJsonRow {
    pub name: String,
    pub product: String,
    pub data: Value,
    pub data_version: i32,
}

#[derive(Debug, Clone, FromRow)]
pub struct ReleaseAssetRow {
    pub name: String,
    pub path: String,
    pub data: Value,
}

#[derive(Debug, Clone, FromRow)]
pub struct OldReleaseRow {
    pub name: String,
    pub product: String,
    pub data: String, // LONGTEXT containing JSON
    pub data_version: i32,
}

#[derive(Debug, Clone, FromRow)]
pub struct EmergencyShutoffRow {
    pub product: String,
    pub channel: String,
}

#[derive(Debug, Clone, FromRow)]
pub struct PinnableReleaseRow {
    pub product: String,
    pub channel: String,
    pub version: String,
    pub mapping: String,
}
