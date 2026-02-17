pub fn match_regex(pattern: &str, value: &str) -> bool {
    // Handle glob-style matching (only * at the end, min length 3)
    if pattern.ends_with('*') {
        let prefix = pattern.trim_end_matches('*');
        if prefix.len() < 3 {
            return false;
        }
        return value.starts_with(prefix);
    }

    // Exact match
    pattern == value
}

pub fn match_channel(rule_channel: &str, query_channel: &str, fallback_channel: Option<&str>) -> bool {
    // Try query channel first
    if match_regex(rule_channel, query_channel) {
        return true;
    }

    // Try fallback channel if available
    if let Some(fb) = fallback_channel {
        if match_regex(rule_channel, fb) {
            return true;
        }
    }

    false
}

pub fn get_fallback_channel(channel: &str) -> Option<String> {
    // Fallback is the part before "-cck-"
    // e.g., "release-cck-something" -> "release"
    if let Some(pos) = channel.find("-cck-") {
        Some(channel[..pos].to_string())
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_match_regex() {
        assert!(match_regex("release", "release"));
        assert!(match_regex("beta*", "beta-test"));
        assert!(match_regex("beta*", "beta"));
        assert!(!match_regex("ab*", "abc")); // Too short prefix
        assert!(match_regex("release*", "release-cdntest"));
    }

    #[test]
    fn test_match_channel() {
        assert!(match_channel("release", "release", None));
        assert!(match_channel("release", "beta", Some("release")));
        assert!(!match_channel("nightly", "release", Some("beta")));
    }

    #[test]
    fn test_get_fallback_channel() {
        assert_eq!(get_fallback_channel("release-cck-something"), Some("release".to_string()));
        assert_eq!(get_fallback_channel("beta-cck-test"), Some("beta".to_string()));
        assert_eq!(get_fallback_channel("esr"), None);
        assert_eq!(get_fallback_channel("nightly"), None);
    }
}
