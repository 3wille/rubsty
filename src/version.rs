pub struct VersionMismatch {
    pub level: VersionLevel,
    pub versions: Vec<RubyVersion>,
}

#[derive(strum_macros::ToString, Debug, Clone)]
pub enum VersionLevel {
    Major,
    Minor,
    Teeny,
    Patch,
}

#[derive(Debug, Clone)]
pub struct RubyVersion {
    pub major: String,
    pub minor: String,
    pub teeny: Option<String>,
    pub patch: Option<String>,
    pub found_in_file: String,
}

impl RubyVersion {
    pub fn print(&self) {
        println!("Detected {} in {}", self, self.found_in_file)
    }

    pub fn from_captures(captures: regex::Captures, filepath: String) -> RubyVersion {
        let major = String::from(captures.name("major").unwrap().as_str());
        let minor = String::from(captures.name("minor").unwrap().as_str());
        let teeny = match captures.name("teeny") {
            Some(teeny) => Some(String::from(teeny.as_str())),
            None => None,
        };
        let patch = match captures.name("patch") {
            Some(patch) => Some(String::from(patch.as_str())),
            None => None,
        };
        RubyVersion {
            major,
            minor,
            teeny,
            patch,
            found_in_file: filepath,
        }
    }
}

impl std::fmt::Display for RubyVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.teeny {
            Some(teeny) => match &self.patch {
                Some(patch) => write!(f, "{}.{}.{}-p{}", self.major, self.minor, teeny, patch),
                None => write!(f, "{}.{}.{}", self.major, self.minor, teeny),
            },
            None => write!(f, "{}.{}", self.major, self.minor),
        }
    }
}

#[test]
fn test_rubyversion_minor_display_impl() {
    let version = RubyVersion {
        major: String::from("1"),
        minor: String::from("2"),
        teeny: None,
        patch: None,
        found_in_file: String::from("a"),
    };
    assert_eq!(format!("{}", version), "1.2")
}

#[test]
fn test_rubyversion_teeny_display_impl() {
    let version = RubyVersion {
        major: String::from("1"),
        minor: String::from("2"),
        teeny: Some(String::from("3")),
        patch: None,
        found_in_file: String::from("a"),
    };
    assert_eq!(format!("{}", version), "1.2.3")
}

#[test]
fn test_rubyversion_patch_display_impl() {
    let version = RubyVersion {
        major: String::from("1"),
        minor: String::from("2"),
        teeny: Some(String::from("3")),
        patch: Some(String::from("4")),
        found_in_file: String::from("a"),
    };
    assert_eq!(format!("{}", version), "1.2.3-p4")
}
