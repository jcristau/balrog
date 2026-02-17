/// Constructs the final XML response with proper escaping
pub fn construct_xml_response(xml: String) -> String {
    // Escape ampersands (but not already-escaped ones)
    // Since Rust regex doesn't support negative lookahead, we do it in two steps
    let mut result = String::with_capacity(xml.len() + xml.len() / 10);
    let mut chars = xml.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '&' {
            // Check if it's already escaped
            let next_chars: String = chars.clone().take(4).collect();
            if next_chars.starts_with("amp;") {
                // Already escaped, keep as-is
                result.push(c);
            } else {
                // Escape it
                result.push_str("&amp;");
            }
        } else {
            result.push(c);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construct_xml_response() {
        let xml = r#"<patch URL="http://example.com?foo=1&bar=2"/>"#;
        let result = construct_xml_response(xml.to_string());

        assert!(result.contains("&amp;"));
        assert!(!result.contains("&bar"));
    }

    #[test]
    fn test_no_double_escape() {
        let xml = r#"<patch URL="http://example.com?foo=1&amp;bar=2"/>"#;
        let result = construct_xml_response(xml.to_string());

        // Should not double-escape
        assert!(!result.contains("&amp;amp;"));
    }
}
