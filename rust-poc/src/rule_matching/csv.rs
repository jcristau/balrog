/// Matches CSV values with optional substring matching
pub fn match_csv(rule_value: &str, query_value: &str, substring: bool) -> bool {
    for item in rule_value.split(',') {
        let item = item.trim();

        if substring {
            if query_value.contains(item) {
                return true;
            }
        } else {
            if query_value == item {
                return true;
            }
        }
    }

    false
}

/// Matches locale values (exact match only, no substring)
pub fn match_locale(rule_locale: &str, query_locale: &str) -> bool {
    match_csv(rule_locale, query_locale, false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_match_csv_exact() {
        assert!(match_csv("en-US,fr,de", "en-US", false));
        assert!(match_csv("en-US,fr,de", "fr", false));
        assert!(!match_csv("en-US,fr,de", "en", false));
    }

    #[test]
    fn test_match_csv_substring() {
        // Substring matching for things like instruction sets (when used with substring=true)
        assert!(match_csv("SSE4_2,AVX", "SSE4_2,AVX2", true));
        assert!(match_csv("SSE4_2,AVX", "SSE4_2", true));
        assert!(!match_csv("SSE4_2,AVX", "SSE3", true));
    }

    #[test]
    fn test_match_csv_exact_iset() {
        // Exact matching for instruction sets (as used in production)
        assert!(match_csv("SSE4_2", "SSE4_2,AVX2", false));
        assert!(match_csv("AVX2", "SSE4_2,AVX2", false));
        assert!(!match_csv("AVX", "SSE4_2,AVX2", false)); // AVX not in query
        assert!(match_csv("SSE4_2,AVX2", "SSE4_2,AVX2", false));
    }

    #[test]
    fn test_match_locale() {
        assert!(match_locale("en-US,fr", "en-US"));
        assert!(!match_locale("en-US,fr", "en"));
    }
}
