use std::fmt::Display;
use serde::{Deserialize, Serialize};
use tokio::net::{TcpStream, UdpSocket};
use crate::errors::Errors;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AdapterType {
    Unknown,
    Direct,
    Reject,
    RejectDrop,
    Compatible,
    Pass,
    Dns,
    Relay,
    Selector,
    Fallback,
    URLTest,
    LoadBalance,
    Shadowsocks,
    ShadowsocksR,
    Snell,
    Socks5,
    Http,
    Vmess,
    Vless,
    Trojan,
    Hysteria,
    Hysteria2,
    WireGuard,
    Tuic,
    Ssh,
    Mieru,
    AnyTLS,
}

impl Default for AdapterType {
    fn default() -> Self {
        AdapterType::Unknown
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Transport{
    TCP4,
    UDP4,
    TCP6,
    UDP6,
    TCP,
    UDP
}

impl Transport {
    pub fn normalized(self, ip_ver: i32) -> Result<Transport, Errors> {
        use Transport::*;
        match (ip_ver, self) {
            (0, t) => Ok(t),

            (4, TCP) | (4, TCP4) => Ok(TCP4),
            (4, UDP) | (4, UDP4) => Ok(UDP4),
            (4, _) => Err(Errors::InvalidIpVersion),

            (6, TCP) => Ok(TCP6),
            (6, UDP) => Ok(UDP6),
            (6, t) => Ok(t),

            _ => Err(Errors::InvalidIpVersion),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HostPort {
    Domain(String, u16),
    Ip(std::net::IpAddr, u16),
}

impl Display for HostPort {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HostPort::Domain(domain, port) => write!(f, "{}:{}", domain, port),
            HostPort::Ip(ip, port) => {
                match ip {
                    std::net::IpAddr::V4(ipv4) => write!(f, "{}:{}", ipv4, port),
                    std::net::IpAddr::V6(ipv6) => write!(f, "[{}]:{}", ipv6, port),
                }
            }
        }
    }
}

// mapped to golang net.Conn
pub enum NetConn {
    TCP(TcpStream),
    UDP(UdpSocket)
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DelayHistory {
    pub time: DateTime<Utc>,
    pub delay: u16,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProxyState {
    alive : bool,
    history: Vec<DelayHistory>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProxyInfo {
    // Extended UDP / UDP-over-TCP
    pub xudp: bool,
    // TCP Fast Open
    pub tfo: bool,
    // Multipath TCP
    pub mp_tcp: bool,
    // Stream Multiplexing
    pub smux: bool,
    pub interface: String,
    pub routing_mark: i32,
    pub dialer_proxy: String,
}

pub trait UDPPacket {
    
}