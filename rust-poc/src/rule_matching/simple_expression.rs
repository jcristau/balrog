/// Matches simple expressions with CSV-style OR and && AND operations
/// Example: "Windows_NT 10.*,Windows_NT 11.*" matches if query contains "Windows_NT 10." OR "Windows_NT 11."
/// Example: "Windows_NT&&10.*" matches if query contains both "Windows_NT" AND "10."
/// Note: Each part does substring matching
pub fn match_simple_expression(rule_value: &str, query_value: &str) -> bool {
    // Split by comma for OR conditions
    for or_part in rule_value.split(',') {
        let or_part = or_part.trim();

        // Check if this OR part contains AND conditions
        let and_parts: Vec<&str> = or_part.split("&&").map(|s| s.trim()).collect();

        // All AND conditions must match (substring match)
        if and_parts.iter().all(|part| query_value.contains(*part)) {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_or() {
        // Substring matching: "Windows_NT 10" is in "Windows_NT 10.0"
        assert!(match_simple_expression("Windows_NT 10,Windows_NT 11", "Windows_NT 10.0"));
        assert!(match_simple_expression("Windows_NT 10,Windows_NT 11", "Windows_NT 11.0"));
        assert!(!match_simple_expression("Windows_NT 10,Windows_NT 11", "Windows_NT 8.0"));
    }

    #[test]
    fn test_simple_and() {
        // Both "Windows_NT" and "10" must be in the query
        assert!(match_simple_expression("Windows_NT&&10", "Windows_NT 10.0"));
        assert!(!match_simple_expression("Windows_NT&&10", "Windows_NT 11.0"));
        assert!(!match_simple_expression("Windows_NT&&11", "Darwin 10.0"));
    }
}
