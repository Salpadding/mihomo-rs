use std::io;
use async_trait::async_trait;
use tokio::net::{TcpStream, UdpSocket};
use crate::component::dialer::{WithOptions};
use crate::errors::Errors;

pub mod metadata;
pub mod context;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AdapterType {
    Direct,
    Reject,
}

pub enum Conn {
    TCP(TcpStream),
    UDP(UdpSocket)
}

#[async_trait]
pub trait ProxyAdapter: Send + Sync {
    fn name(&self) -> &str;
    fn adapter_type(&self) -> AdapterType;
    fn addr(&self) -> &str;
    async fn dial_context(&self, ctx: &context::Context, network: Transport, addr: &HostPort) -> Result<Conn, Errors>;
    fn dial_options(&self) -> Vec<WithOptions>;
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
    IP(std::net::IpAddr, u16),
}

#[async_trait]
pub trait InboundListener {
    fn name(&self) -> &str;
    async fn listen(&self, tunnel: &dyn Tunnel) -> Result<(), Errors>;
}

#[async_trait]
pub trait Tunnel: Send + Sync {
    async fn handle_tcp_conn(&self, conn: TcpStream, metadata: Box<metadata::Metadata>) -> Result<(), Errors>;

    fn handle_udp_packet(&self, socket: UdpSocket, metadata: &metadata::Metadata);

    fn handle_error(&self, err: io::Error);
}