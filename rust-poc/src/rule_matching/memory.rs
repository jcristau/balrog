use super::comparison::{get_op, int_compare};

pub fn match_memory(rule_memory: i64, query_memory: Option<i64>) -> bool {
    let query_mem = match query_memory {
        Some(m) => m,
        None => return true, // If query memory is None, match succeeds
    };

    // Rule memory might have an operator prefix
    let rule_str = rule_memory.to_string();
    let (op, value_str) = get_op(&rule_str);

    let rule_val = match value_str.parse::<i64>() {
        Ok(v) => v,
        Err(_) => return false,
    };

    // Compare query against rule (queryMemory op ruleMemory)
    int_compare(&op, query_mem, rule_val)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_match_memory() {
        // Exact match
        assert!(match_memory(8192, Some(8192)));
        // If query memory is None, it matches (permissive)
        assert!(match_memory(8192, None));
        // Different values don't match without operator
        assert!(!match_memory(8192, Some(16384)));
        assert!(!match_memory(8192, Some(4096)));
    }
}
