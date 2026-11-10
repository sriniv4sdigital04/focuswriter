//! Static build info for Focuswriter.

pub const BUILD_VERSION: &str  = "v1.8.20";
pub const BUILD_CHANNEL: &str  = "stable";
pub const BUILD_COMMIT: &str   = "243050085f7d";
pub const BUILD_TIMESTAMP: u64 = 1755827917;

pub fn version_tuple() -> (u16, u16, u16) {
    let mut parts = BUILD_VERSION.trim_start_matches('v').split('.');
    let major = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    let minor = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    let patch = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    (major, minor, patch)
}

pub fn user_agent() -> String {
    format!("{}/{} ({})", "Focuswriter", BUILD_VERSION, BUILD_CHANNEL)
}
