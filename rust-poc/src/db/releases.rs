use sqlx::MySqlPool;
use serde_json::{Value, json};
use crate::db::models::{ReleasesJsonRow, ReleaseAssetRow, OldReleaseRow};
use crate::error::AppError;

/// Fetches a release blob by name
/// Tries new-style (releases_json + release_assets) first, then falls back to old-style (releases)
pub async fn get_release(pool: &MySqlPool, name: &str) -> Result<Value, AppError> {
    // Try new-style first
    if let Some(blob) = get_release_new_style(pool, name).await? {
        return Ok(blob);
    }

    // Fall back to old-style
    get_release_old_style(pool, name).await
}

/// Gets release from releases_json table and merges in assets from release_assets
async fn get_release_new_style(pool: &MySqlPool, name: &str) -> Result<Option<Value>, AppError> {
    let base_row: Option<ReleasesJsonRow> = sqlx::query_as(
        "SELECT name, product, data, data_version FROM releases_json WHERE name = ?"
    )
    .bind(name)
    .fetch_optional(pool)
    .await?;

    let base_row = match base_row {
        Some(row) => row,
        None => return Ok(None),
    };

    // Get all assets for this release
    let assets: Vec<ReleaseAssetRow> = sqlx::query_as(
        "SELECT name, path, data FROM release_assets WHERE name = ?"
    )
    .bind(name)
    .fetch_all(pool)
    .await?;

    // Start with base data
    let mut merged = base_row.data.clone();

    // Merge each asset into the base data at its path
    for asset in assets {
        merge_asset_at_path(&mut merged, &asset.path, asset.data);
    }

    Ok(Some(merged))
}

/// Merges asset data into the blob at the specified dot-delimited path
fn merge_asset_at_path(blob: &mut Value, path: &str, data: Value) {
    let parts: Vec<&str> = path.trim_start_matches('.').split('.').collect();

    if parts.is_empty() {
        return;
    }

    // Navigate/create path
    let mut current = blob;
    for part in &parts[..parts.len() - 1] {
        let part_str = part.to_string();

        // Check if we need to create this level
        let needs_creation = !current
            .as_object()
            .map_or(false, |obj| obj.contains_key(*part));

        if needs_creation {
            if let Some(obj) = current.as_object_mut() {
                obj.insert(part_str.clone(), json!({}));
            }
        }

        // Navigate to next level
        current = current
            .as_object_mut()
            .and_then(|obj| obj.get_mut(&part_str))
            .expect("Failed to navigate path");
    }

    // Set the final value
    let final_key = parts[parts.len() - 1].to_string();

    if let Some(obj) = current.as_object_mut() {
        // Check if we should deep merge
        let should_merge = obj
            .get(&final_key)
            .and_then(|v| v.as_object())
            .is_some()
            && data.as_object().is_some();

        if should_merge {
            let existing_obj = obj.get(&final_key).unwrap().as_object().unwrap();
            let new_obj = data.as_object().unwrap();

            let mut merged_obj = existing_obj.clone();
            for (k, v) in new_obj {
                merged_obj.insert(k.clone(), v.clone());
            }
            obj.insert(final_key, Value::Object(merged_obj));
        } else {
            obj.insert(final_key, data);
        }
    }
}

/// Gets release from old-style releases table
async fn get_release_old_style(pool: &MySqlPool, name: &str) -> Result<Value, AppError> {
    let row: Option<OldReleaseRow> = sqlx::query_as(
        "SELECT name, product, data, data_version FROM releases WHERE name = ?"
    )
    .bind(name)
    .fetch_optional(pool)
    .await?;

    let row = row.ok_or_else(|| AppError::NotFound(format!("Release not found: {}", name)))?;

    // Parse LONGTEXT JSON
    let data: Value = serde_json::from_str(&row.data)
        .map_err(|e| AppError::Internal(format!("Failed to parse release JSON: {}", e)))?;

    Ok(data)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_asset_at_path() {
        let mut blob = json!({
            "name": "Firefox-130.0-build1",
            "platforms": {}
        });

        let asset = json!({
            "en-US": {
                "buildID": "20240801000000"
            }
        });

        merge_asset_at_path(&mut blob, ".platforms.WINNT_x86_64-msvc.locales", asset);

        assert_eq!(
            blob["platforms"]["WINNT_x86_64-msvc"]["locales"]["en-US"]["buildID"],
            "20240801000000"
        );
    }
}
