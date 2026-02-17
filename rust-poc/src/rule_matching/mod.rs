pub mod comparison;
pub mod version;
pub mod channel;
pub mod buildid;
pub mod memory;
pub mod simple_expression;
pub mod csv;
pub mod boolean;

use crate::db::models::RuleRow;
use crate::update::query::UpdateQuery;

use channel::{match_channel, get_fallback_channel};
use version::MozillaVersion;
use comparison::{get_op, version_compare};
use buildid::match_build_id;
use memory::match_memory;
use simple_expression::match_simple_expression;
use csv::{match_csv, match_locale};
use boolean::match_boolean;

/// Performs in-memory filtering of rules (Phase 2 of rule matching)
pub fn filter_matching_rules(rules: Vec<RuleRow>, query: &UpdateQuery) -> Vec<RuleRow> {
    let fallback_channel = get_fallback_channel(&query.channel);

    rules.into_iter()
        .filter(|rule| {
            // Channel matching
            if let Some(ref rule_channel) = rule.channel {
                if !match_channel(rule_channel, &query.channel, fallback_channel.as_deref()) {
                    return false;
                }
            }

            // Version matching
            if let Some(ref rule_version) = rule.version {
                let (op, version_str) = get_op(rule_version);

                if let (Ok(query_ver), Ok(rule_ver)) = (
                    MozillaVersion::parse(&query.version),
                    MozillaVersion::parse(version_str)
                ) {
                    if !version_compare(&op, &query_ver, &rule_ver) {
                        return false;
                    }
                } else {
                    return false;
                }
            }

            // BuildID matching
            if let Some(ref rule_build_id) = rule.build_id {
                if !match_build_id(rule_build_id, &query.build_id) {
                    return false;
                }
            }

            // Memory matching
            if let Some(rule_memory) = rule.memory {
                if !match_memory(rule_memory, query.memory) {
                    return false;
                }
            }

            // OS Version matching
            if let Some(ref rule_os_version) = rule.os_version {
                if !match_simple_expression(rule_os_version, &query.os_version) {
                    return false;
                }
            }

            // Instruction Set matching
            if let Some(ref rule_instruction_set) = rule.instruction_set {
                if let Some(ref query_iset) = query.instruction_set {
                    if !match_csv(rule_instruction_set, query_iset, false) {
                        return false;
                    }
                } else {
                    return false;
                }
            }

            // Distribution matching
            if let Some(ref rule_dist) = rule.distribution {
                if !match_csv(rule_dist, &query.distribution, false) {
                    return false;
                }
            }

            // Locale matching
            if let Some(ref rule_locale) = rule.locale {
                if !match_locale(rule_locale, &query.locale) {
                    return false;
                }
            }

            // MIG64 boolean matching
            if let Some(rule_mig64) = rule.mig64 {
                if !match_boolean(Some(rule_mig64), query.mig64) {
                    return false;
                }
            }

            // JAWS boolean matching
            if let Some(rule_jaws) = rule.jaws {
                if !match_boolean(Some(rule_jaws), query.jaws) {
                    return false;
                }
            }

            true
        })
        .collect()
}

/// Gets the highest priority matching rule
pub fn get_best_matching_rule(mut rules: Vec<RuleRow>) -> Option<RuleRow> {
    if rules.is_empty() {
        return None;
    }

    // Sort by priority descending
    rules.sort_by(|a, b| b.priority.cmp(&a.priority));

    // Return highest priority rule
    rules.into_iter().next()
}
