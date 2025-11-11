use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DNSPrefer {
    Unknown,
    DualStack,
    IPv4Only,
    IPv6Only,
    IPv4Prefer,
    IPv6Prefer,
}

impl Default for DNSPrefer {
    fn default() -> Self {
        DNSPrefer::Unknown
    }
}