use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Platform {
    Windows,
    MacIntel,
    MacArm,
    IOS,
    Android32,
    Android64,
}

impl Platform {
    pub fn all() -> &'static [Platform] {
        &[
            Platform::Windows,
            Platform::MacIntel,
            Platform::MacArm,
            Platform::IOS,
            Platform::Android32,
            Platform::Android64,
        ]
    }

    pub fn cfg_condition(self) -> &'static str {
        match self {
            Platform::Windows => "target_os = \"windows\"",
            Platform::MacIntel => "all(target_os = \"macos\", target_arch = \"x86_64\")",
            Platform::MacArm => "all(target_os = \"macos\", target_arch = \"aarch64\")",
            Platform::IOS => "target_os = \"ios\"",
            Platform::Android32 => "all(target_os = \"android\", target_arch = \"arm\")",
            Platform::Android64 => "all(target_os = \"android\", target_arch = \"aarch64\")",
        }
    }

    pub fn is_macos(self) -> bool {
        matches!(self, Platform::MacIntel | Platform::MacArm)
    }
}

impl fmt::Display for Platform {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Platform::Windows => write!(f, "windows"),
            Platform::MacIntel => write!(f, "mac_intel"),
            Platform::MacArm => write!(f, "mac_arm"),
            Platform::IOS => write!(f, "ios"),
            Platform::Android32 => write!(f, "android32"),
            Platform::Android64 => write!(f, "android64"),
        }
    }
}
