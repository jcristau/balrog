use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Force {
    MainMapping,
    FallbackMapping,
}

#[derive(Debug, Clone)]
pub struct UpdateQuery {
    pub product: String,
    pub version: String,
    pub build_id: String,
    pub build_target: String,
    pub locale: String,
    pub channel: String,
    pub os_version: String,
    pub distribution: String,
    pub dist_version: String,
    pub header_architecture: Option<String>,
    pub instruction_set: Option<String>,
    pub memory: Option<i64>,
    pub jaws: Option<bool>,
    pub mig64: Option<bool>,
    pub force: Option<Force>,
}

impl UpdateQuery {
    pub fn from_path_and_query(
        product: String,
        version: String,
        build_id: String,
        build_target: String,
        locale: String,
        channel: String,
        os_version: String,
        system_capabilities: String,
        distribution: String,
        dist_version: String,
        query_params: HashMap<String, String>,
        user_agent: Option<String>,
    ) -> Self {
        // Parse system capabilities (ISET:SSE4_2,MEM:8192,JAWS:1 format)
        let (instruction_set, memory, jaws) = parse_system_capabilities(&system_capabilities);

        // Compute header architecture from buildTarget and User-Agent
        let header_architecture = get_header_architecture(&build_target, user_agent.as_deref());

        // Parse force parameter
        let force = query_params.get("force").and_then(|f| match f.as_str() {
            "1" => Some(Force::MainMapping),
            "-1" => Some(Force::FallbackMapping),
            _ => None,
        });

        // Parse mig64 parameter
        let mig64 = query_params.get("mig64").map(|m| m == "1");

        UpdateQuery {
            product,
            version,
            build_id,
            build_target,
            locale,
            channel,
            os_version,
            distribution,
            dist_version,
            header_architecture,
            instruction_set,
            memory,
            jaws,
            mig64,
            force,
        }
    }
}

/// Parses system capabilities string
/// Modern format: ISET:SSE4_2,MEM:8192,JAWS:1
/// Old format: SSE,8192
fn parse_system_capabilities(caps: &str) -> (Option<String>, Option<i64>, Option<bool>) {
    let mut instruction_set = None;
    let mut memory = None;
    let mut jaws = None;

    // Check for modern format (contains ISET:, MEM:, or JAWS:)
    if caps.contains("ISET:") || caps.contains("MEM:") || caps.contains("JAWS:") {
        for part in caps.split(',') {
            let part = part.trim();

            if let Some(iset) = part.strip_prefix("ISET:") {
                instruction_set = Some(iset.to_string());
            } else if let Some(mem) = part.strip_prefix("MEM:") {
                memory = mem.parse::<i64>().ok();
            } else if let Some(jaws_val) = part.strip_prefix("JAWS:") {
                jaws = Some(jaws_val == "1");
            }
        }
    } else {
        // Old format: first part is instruction set, second is memory
        let parts: Vec<&str> = caps.split(',').collect();

        if !parts.is_empty() && !parts[0].is_empty() {
            instruction_set = Some(parts[0].to_string());
        }

        if parts.len() > 1 {
            memory = parts[1].parse::<i64>().ok();
        }
    }

    (instruction_set, memory, jaws)
}

/// Computes header architecture from build target and User-Agent
fn get_header_architecture(build_target: &str, user_agent: Option<&str>) -> Option<String> {
    // For Windows builds, check User-Agent for WOW64 or Win64
    if build_target.starts_with("WINNT") {
        if let Some(ua) = user_agent {
            if ua.contains("WOW64") {
                return Some("x86".to_string());
            } else if ua.contains("Win64") {
                return Some("x86_64".to_string());
            }
        }

        // Default based on build target
        if build_target.contains("x86_64") {
            return Some("x86_64".to_string());
        } else if build_target.contains("x86-") {
            return Some("x86".to_string());
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_system_capabilities_modern() {
        let (iset, mem, jaws) = parse_system_capabilities("ISET:SSE4_2,MEM:8192,JAWS:1");

        assert_eq!(iset, Some("SSE4_2".to_string()));
        assert_eq!(mem, Some(8192));
        assert_eq!(jaws, Some(true));
    }

    #[test]
    fn test_parse_system_capabilities_old() {
        let (iset, mem, jaws) = parse_system_capabilities("SSE4_2,8192");

        assert_eq!(iset, Some("SSE4_2".to_string()));
        assert_eq!(mem, Some(8192));
        assert_eq!(jaws, None);
    }

    #[test]
    fn test_get_header_architecture() {
        assert_eq!(
            get_header_architecture("WINNT_x86_64-msvc", Some("Mozilla/5.0 (Windows NT 10.0; Win64; x64)")),
            Some("x86_64".to_string())
        );

        assert_eq!(
            get_header_architecture("WINNT_x86_64-msvc", Some("Mozilla/5.0 (Windows NT 10.0; WOW64)")),
            Some("x86".to_string())
        );
    }
}
