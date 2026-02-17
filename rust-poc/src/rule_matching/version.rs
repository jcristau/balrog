use std::cmp::Ordering;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MozillaVersion {
    PostModern(PostModernVersion),
    Glob(GlobVersion),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PostModernVersion {
    pub major: u32,
    pub minor: Option<u32>,
    pub patch: Option<u32>,
    pub prerelease: Option<String>, // "a1", "b2", etc.
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GlobVersion {
    pub major: u32,
}

impl MozillaVersion {
    pub fn parse(version_str: &str) -> Result<Self, String> {
        // Check if it's a glob pattern (e.g., "70.*")
        if version_str.ends_with(".*") {
            let major_str = version_str.trim_end_matches(".*");
            let major = major_str.parse::<u32>()
                .map_err(|_| format!("Invalid glob version: {}", version_str))?;
            return Ok(MozillaVersion::Glob(GlobVersion { major }));
        }

        // Parse as PostModern version
        let mut parts = version_str.split('.');

        let major_str = parts.next()
            .ok_or_else(|| format!("Invalid version: {}", version_str))?;

        // Check for prerelease in major (e.g., "130a1")
        let (major, prerelease) = if let Some(pos) = major_str.find(|c: char| c == 'a' || c == 'b') {
            let (maj, pre) = major_str.split_at(pos);
            let major = maj.parse::<u32>()
                .map_err(|_| format!("Invalid major version: {}", maj))?;
            (major, Some(pre.to_string()))
        } else {
            let major = major_str.parse::<u32>()
                .map_err(|_| format!("Invalid major version: {}", major_str))?;
            (major, None)
        };

        let minor = parts.next().and_then(|s| s.parse::<u32>().ok());
        let patch = parts.next().and_then(|s| s.parse::<u32>().ok());

        Ok(MozillaVersion::PostModern(PostModernVersion {
            major,
            minor,
            patch,
            prerelease,
        }))
    }
}

impl PartialOrd for MozillaVersion {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for MozillaVersion {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (MozillaVersion::PostModern(a), MozillaVersion::PostModern(b)) => a.cmp(b),
            (MozillaVersion::Glob(a), MozillaVersion::Glob(b)) => a.major.cmp(&b.major),
            // Glob versions only compare major
            (MozillaVersion::PostModern(a), MozillaVersion::Glob(b)) => a.major.cmp(&b.major),
            (MozillaVersion::Glob(a), MozillaVersion::PostModern(b)) => a.major.cmp(&b.major),
        }
    }
}

impl PartialOrd for PostModernVersion {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PostModernVersion {
    fn cmp(&self, other: &Self) -> Ordering {
        // Compare major
        match self.major.cmp(&other.major) {
            Ordering::Equal => {}
            ord => return ord,
        }

        // Compare minor (None is less than Some)
        match (&self.minor, &other.minor) {
            (Some(a), Some(b)) => match a.cmp(b) {
                Ordering::Equal => {}
                ord => return ord,
            },
            (None, Some(_)) => return Ordering::Less,
            (Some(_), None) => return Ordering::Greater,
            (None, None) => {}
        }

        // Compare patch
        match (&self.patch, &other.patch) {
            (Some(a), Some(b)) => match a.cmp(b) {
                Ordering::Equal => {}
                ord => return ord,
            },
            (None, Some(_)) => return Ordering::Less,
            (Some(_), None) => return Ordering::Greater,
            (None, None) => {}
        }

        // Compare prerelease (None is greater than Some - release > prerelease)
        match (&self.prerelease, &other.prerelease) {
            (Some(a), Some(b)) => a.cmp(b),
            (None, Some(_)) => Ordering::Greater,
            (Some(_), None) => Ordering::Less,
            (None, None) => Ordering::Equal,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_postmodern() {
        let v = MozillaVersion::parse("130.0.1").unwrap();
        match v {
            MozillaVersion::PostModern(pv) => {
                assert_eq!(pv.major, 130);
                assert_eq!(pv.minor, Some(0));
                assert_eq!(pv.patch, Some(1));
                assert_eq!(pv.prerelease, None);
            }
            _ => panic!("Expected PostModern version"),
        }
    }

    #[test]
    fn test_parse_glob() {
        let v = MozillaVersion::parse("70.*").unwrap();
        match v {
            MozillaVersion::Glob(gv) => {
                assert_eq!(gv.major, 70);
            }
            _ => panic!("Expected Glob version"),
        }
    }

    #[test]
    fn test_version_ordering() {
        let v1 = MozillaVersion::parse("130.0").unwrap();
        let v2 = MozillaVersion::parse("130.0.1").unwrap();
        let v3 = MozillaVersion::parse("131.0").unwrap();

        assert!(v1 < v2);
        assert!(v2 < v3);
    }
}
