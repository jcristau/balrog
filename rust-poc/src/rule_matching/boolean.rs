/// Matches boolean values using 3x3 truth table
/// Rule value: Some(true), Some(false), or None
/// Query value: Some(true), Some(false), or None
pub fn match_boolean(rule_value: Option<bool>, query_value: Option<bool>) -> bool {
    match (rule_value, query_value) {
        // Rule is None: matches anything
        (None, _) => true,

        // Rule is Some(true): matches only Some(true)
        (Some(true), Some(true)) => true,
        (Some(true), _) => false,

        // Rule is Some(false): matches Some(false) or None
        (Some(false), Some(true)) => false,
        (Some(false), _) => true,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_boolean_matching() {
        // Rule None matches anything
        assert!(match_boolean(None, None));
        assert!(match_boolean(None, Some(true)));
        assert!(match_boolean(None, Some(false)));

        // Rule true matches only true
        assert!(match_boolean(Some(true), Some(true)));
        assert!(!match_boolean(Some(true), Some(false)));
        assert!(!match_boolean(Some(true), None));

        // Rule false matches false or None
        assert!(match_boolean(Some(false), Some(false)));
        assert!(match_boolean(Some(false), None));
        assert!(!match_boolean(Some(false), Some(true)));
    }
}
