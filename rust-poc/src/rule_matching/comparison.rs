use super::version::MozillaVersion;

#[derive(Debug, PartialEq)]
pub enum Operator {
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
    Equal,
}

pub fn get_op(value: &str) -> (Operator, &str) {
    if let Some(rest) = value.strip_prefix("<=") {
        (Operator::LessThanOrEqual, rest)
    } else if let Some(rest) = value.strip_prefix('<') {
        (Operator::LessThan, rest)
    } else if let Some(rest) = value.strip_prefix(">=") {
        (Operator::GreaterThanOrEqual, rest)
    } else if let Some(rest) = value.strip_prefix('>') {
        (Operator::GreaterThan, rest)
    } else {
        (Operator::Equal, value)
    }
}

pub fn string_compare(op: &Operator, value1: &str, value2: &str) -> bool {
    match op {
        Operator::LessThan => value1 < value2,
        Operator::LessThanOrEqual => value1 <= value2,
        Operator::GreaterThan => value1 > value2,
        Operator::GreaterThanOrEqual => value1 >= value2,
        Operator::Equal => value1 == value2,
    }
}

pub fn int_compare(op: &Operator, value1: i64, value2: i64) -> bool {
    match op {
        Operator::LessThan => value1 < value2,
        Operator::LessThanOrEqual => value1 <= value2,
        Operator::GreaterThan => value1 > value2,
        Operator::GreaterThanOrEqual => value1 >= value2,
        Operator::Equal => value1 == value2,
    }
}

pub fn version_compare(op: &Operator, version1: &MozillaVersion, version2: &MozillaVersion) -> bool {
    match op {
        Operator::LessThan => version1 < version2,
        Operator::LessThanOrEqual => version1 <= version2,
        Operator::GreaterThan => version1 > version2,
        Operator::GreaterThanOrEqual => version1 >= version2,
        Operator::Equal => version1 == version2,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_op() {
        assert_eq!(get_op("<5"), (Operator::LessThan, "5"));
        assert_eq!(get_op("<=10"), (Operator::LessThanOrEqual, "10"));
        assert_eq!(get_op(">3"), (Operator::GreaterThan, "3"));
        assert_eq!(get_op(">=7"), (Operator::GreaterThanOrEqual, "7"));
        assert_eq!(get_op("42"), (Operator::Equal, "42"));
    }

    #[test]
    fn test_string_compare() {
        assert!(string_compare(&Operator::LessThan, "abc", "def"));
        assert!(string_compare(&Operator::Equal, "foo", "foo"));
        assert!(string_compare(&Operator::GreaterThan, "xyz", "abc"));
    }

    #[test]
    fn test_int_compare() {
        assert!(int_compare(&Operator::LessThan, 5, 10));
        assert!(int_compare(&Operator::GreaterThanOrEqual, 10, 10));
        assert!(!int_compare(&Operator::Equal, 5, 10));
    }
}
