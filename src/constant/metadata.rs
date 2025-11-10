use serde::{Deserialize, Serialize};
use std::net;
use derive_builder::Builder;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Network {
    Unknown,
    TCP,
    UDP,
    ALLNet2,
    InvalidNet,
}

impl Default for Network {
    fn default() -> Self {
        Network::Unknown
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize, Eq)]
pub enum SourceType {
    Unknown,
    HTTP,
    HTTPS,
    SOCKS4,
    SOCKS5,
    SHADOWSOCKS,
    VMESS,
    VLESS,
    REDIR,
    TPROXY,
    TROJAN,
    TUNNEL,
    TUN,
    TUIC,
    HYSTERIA2,
    ANYTLS,
    INNER,
}

impl Default for SourceType {
    fn default() -> Self {
        SourceType::Unknown
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, Builder)]
#[serde(rename_all = "camelCase")]
pub struct Metadata {
    pub network: Network,
    pub r#type: SourceType,

    #[serde( skip_serializing_if = "Option::is_none" )]
    pub source_ip: Option<net::IpAddr>,
    #[serde( skip_serializing_if = "Option::is_none" )]
    pub destination_ip: Option<net::IpAddr>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub source_geo_ip: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub destination_geo_ip: Vec<String>,

    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub source_ipasn: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub destination_ipasn: String,

    #[serde(default)]
    pub source_port: u16,
    #[serde(default)]
    pub destination_port: u16,

    #[serde(skip_serializing_if = "Option::is_none" )]
    pub inbound_ip: Option<net::IpAddr>,
    #[serde(default)]
    pub inbound_port: u16,

    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub inbound_name: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub inbound_user: String,

    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub host: String,

    #[serde(default)]
    pub dns_mode: String,

    #[serde(default)]
    pub uid: u32,

    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub process: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub process_path: String,

    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub special_proxy: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub special_rules: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub remote_destination: String,

    #[serde(default)]
    pub dscp: u8,

    #[serde(skip)]
    pub raw_source_addr: Option<String>,
    #[serde(skip)]
    pub raw_destination_addr: Option<String>,

    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub sniff_host: String,
}

impl Metadata {
    // TODO: implement unwrap_ip, unwrap (IPv4-mapped IPv6 address) into ip4
    pub fn unwrap_ip(&mut self) {

    }
}