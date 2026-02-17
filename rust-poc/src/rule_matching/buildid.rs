use super::comparison::{get_op, string_compare};

pub fn match_build_id(rule_build_id: &str, query_build_id: &str) -> bool {
    let (op, value) = get_op(rule_build_id);
    string_compare(&op, query_build_id, value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_match_build_id() {
        assert!(match_build_id("20240801000000", "20240801000000"));
        assert!(match_build_id("<20240801000000", "20240731000000"));
        assert!(match_build_id(">=20240801000000", "20240801000000"));
        assert!(!match_build_id(">20240801000000", "20240801000000"));
    }
}
