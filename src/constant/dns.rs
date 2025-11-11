use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DNSPrefer {
    Unknown,
    DualStack,
    IpV4Only,
    IpV6Only,
    IpV4Prefer,
    IpV6Prefer,
}

impl Default for DNSPrefer {
    fn default() -> Self {
        DNSPrefer::Unknown
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum DNSMode {
    Normal,
    FakeIp,
    Mapping,
    Hosts
}

impl Default for DNSMode {
    fn default() -> Self {
        DNSMode::Normal
    }
}