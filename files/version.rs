//! Static build info for Baby Audio Crystalline.

pub const BUILD_VERSION: &str  = "v1.0.20";
pub const BUILD_CHANNEL: &str  = "stable";
pub const BUILD_COMMIT: &str   = "00f01411bf15";
pub const BUILD_TIMESTAMP: u64 = 1757989965;

pub fn version_tuple() -> (u16, u16, u16) {
    let mut parts = BUILD_VERSION.trim_start_matches('v').split('.');
    let major = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    let minor = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    let patch = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    (major, minor, patch)
}

pub fn user_agent() -> String {
    format!("{}/{} ({})", "Baby Audio Crystalline", BUILD_VERSION, BUILD_CHANNEL)
}
