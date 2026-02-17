use sqlx::MySqlPool;
use crate::db::models::RuleRow;
use crate::error::AppError;

/// Phase 1 SQL query: Filter rules by exact-match and NULL-match columns
pub async fn get_rules_from_db(
    pool: &MySqlPool,
    product: &str,
    build_target: &str,
    header_architecture: Option<&str>,
    dist_version: &str,
) -> Result<Vec<RuleRow>, AppError> {
    let header_arch_str = header_architecture.unwrap_or("");

    let rules = sqlx::query_as::<_, RuleRow>(
        r#"
        SELECT
            rule_id,
            priority,
            mapping,
            fallbackMapping as fallback_mapping,
            backgroundRate as background_rate,
            product,
            version,
            buildID as build_id,
            channel,
            buildTarget as build_target,
            locale,
            osVersion as os_version,
            instructionSet as instruction_set,
            memory,
            CASE WHEN mig64 = 1 THEN true WHEN mig64 = 0 THEN false ELSE NULL END as mig64,
            CASE WHEN jaws = 1 THEN true WHEN jaws = 0 THEN false ELSE NULL END as jaws,
            distribution,
            distVersion as dist_version,
            headerArchitecture as header_architecture,
            data_version
        FROM rules
        WHERE
            (product = ? OR product IS NULL)
            AND (buildTarget = ? OR buildTarget IS NULL)
            AND (headerArchitecture = ? OR headerArchitecture IS NULL OR headerArchitecture = '')
            AND (distVersion = ? OR distVersion IS NULL)
        "#,
    )
    .bind(product)
    .bind(build_target)
    .bind(header_arch_str)
    .bind(dist_version)
    .fetch_all(pool)
    .await?;

    Ok(rules)
}
