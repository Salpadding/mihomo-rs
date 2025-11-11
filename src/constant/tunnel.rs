use std::io;
use async_trait::async_trait;
use tokio::net::{TcpStream, UdpSocket};
use crate::constant::metadata;
use crate::constant::types::UDPPacket;
use crate::errors::Errors;

#[async_trait]
pub trait Tunnel: Send + Sync {
    async fn handle_tcp_conn(&self, conn: TcpStream, metadata: Box<metadata::Metadata>) -> Result<(), Errors>;

    fn handle_udp_packet(&self, packet: &dyn UDPPacket, metadata: Box<metadata::Metadata>);

    fn handle_error(&self, err: io::Error);
}
