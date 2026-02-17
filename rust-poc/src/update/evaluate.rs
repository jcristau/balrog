use sqlx::MySqlPool;
use crate::db::{rules, releases, emergency_shutoffs, pinnable_releases};
use crate::rule_matching::{filter_matching_rules, get_best_matching_rule};
use crate::blobs::{create_blob, BlobType};
use crate::blobs::base::ServeUpdate;
use crate::update::query::{UpdateQuery, Force};
use crate::rule_matching::channel::get_fallback_channel;
use crate::error::AppError;
use rand::Rng;

pub struct EvaluateResult {
    pub blob: Option<BlobType>,
    pub rule_id: Option<i32>,
    pub rule_data_version: Option<i32>,
}

/// Evaluates rules and returns the matching blob
pub async fn evaluate_rules(
    pool: &MySqlPool,
    query: &UpdateQuery,
) -> Result<EvaluateResult, AppError> {
    // Check emergency shutoffs for main channel
    if emergency_shutoffs::is_emergency_shutoff(pool, &query.product, &query.channel).await? {
        return Ok(EvaluateResult {
            blob: None,
            rule_id: None,
            rule_data_version: None,
        });
    }

    // Check emergency shutoffs for fallback channel
    if let Some(fallback_channel) = get_fallback_channel(&query.channel) {
        if emergency_shutoffs::is_emergency_shutoff(pool, &query.product, &fallback_channel).await? {
            return Ok(EvaluateResult {
                blob: None,
                rule_id: None,
                rule_data_version: None,
            });
        }
    }

    // Phase 1: Get rules from database
    let db_rules = rules::get_rules_from_db(
        pool,
        &query.product,
        &query.build_target,
        query.header_architecture.as_deref(),
        &query.dist_version,
    ).await?;

    // Phase 2: Filter rules in memory
    let matching_rules = filter_matching_rules(db_rules, query);

    // Get best matching rule
    let rule = match get_best_matching_rule(matching_rules) {
        Some(r) => r,
        None => {
            return Ok(EvaluateResult {
                blob: None,
                rule_id: None,
                rule_data_version: None,
            });
        }
    };

    // Handle backgroundRate
    let use_fallback = should_use_fallback(&rule, query)?;

    // Determine which mapping to use
    let mapping_name = if use_fallback {
        rule.fallback_mapping.as_ref()
    } else {
        rule.mapping.as_ref()
    };

    let mapping_name = match mapping_name {
        Some(m) => m,
        None => {
            return Ok(EvaluateResult {
                blob: None,
                rule_id: Some(rule.rule_id),
                rule_data_version: Some(rule.data_version),
            });
        }
    };

    // Fetch release blob
    let release_data = releases::get_release(pool, mapping_name).await?;

    // Create blob
    let blob = create_blob(release_data)?;

    // Check if we should serve the update
    let should_serve = blob.as_blob().should_serve_update(query)?;

    match should_serve {
        ServeUpdate::Yes => {
            Ok(EvaluateResult {
                blob: Some(blob),
                rule_id: Some(rule.rule_id),
                rule_data_version: Some(rule.data_version),
            })
        }
        ServeUpdate::No => {
            Ok(EvaluateResult {
                blob: None,
                rule_id: Some(rule.rule_id),
                rule_data_version: Some(rule.data_version),
            })
        }
        ServeUpdate::Maybe => {
            // Check pinnable releases
            if let Some(pinned_mapping) = pinnable_releases::get_pinnable_release(
                pool,
                &query.product,
                &query.channel,
                &query.version,
            ).await? {
                // Fetch pinned release
                let pinned_data = releases::get_release(pool, &pinned_mapping).await?;
                let pinned_blob = create_blob(pinned_data)?;

                Ok(EvaluateResult {
                    blob: Some(pinned_blob),
                    rule_id: Some(rule.rule_id),
                    rule_data_version: Some(rule.data_version),
                })
            } else {
                // No pinned release, serve the original blob
                Ok(EvaluateResult {
                    blob: Some(blob),
                    rule_id: Some(rule.rule_id),
                    rule_data_version: Some(rule.data_version),
                })
            }
        }
    }
}

/// Determines if we should use fallback mapping based on backgroundRate
fn should_use_fallback(rule: &crate::db::models::RuleRow, query: &UpdateQuery) -> Result<bool, AppError> {
    // If force is MainMapping, always use main mapping
    if query.force == Some(Force::MainMapping) {
        return Ok(false);
    }

    // If force is FallbackMapping, always use fallback
    if query.force == Some(Force::FallbackMapping) {
        return Ok(true);
    }

    // If backgroundRate is 100, always use main mapping
    if rule.background_rate >= 100 {
        return Ok(false);
    }

    // Roll dice
    let mut rng = rand::thread_rng();
    let roll: i32 = rng.gen_range(0..100);

    Ok(roll >= rule.background_rate)
}
