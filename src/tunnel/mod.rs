use crate::adapter::outbound;
use crate::constant;
use crate::constant::Transport::TCP;
use crate::constant::context::Context;
use crate::constant::metadata::Metadata;
use crate::constant::{NetConn, HostPort, ProxyAdapter};
use crate::errors::Errors;
use async_trait::async_trait;
use std::io::Error;
use std::sync::{Arc, RwLock};
use tokio::net::{TcpStream, UdpSocket};
use crate::constant::types::UDPPacket;

pub struct Tunnel {}

impl Tunnel {
    pub fn new() -> Self {
        Tunnel {}
    }
}

#[async_trait]
impl constant::Tunnel for Tunnel {
    async fn handle_tcp_conn(
        &self,
        mut inbound: TcpStream,
        metadata: Box<Metadata>,
    ) -> Result<(), Errors> {
        let adapter = self.resolve_metadata(&metadata);
        let host_port = if let Some(hp) = &metadata.destination_ip {
            HostPort::Ip(hp.clone(), metadata.destination_port)
        } else {
            HostPort::Domain(metadata.host.clone(), metadata.destination_port)
        };
        let ctx = Context::new(metadata, host_port);
        let conn = adapter
            .dial_context(
                &ctx,
                TCP,
                ctx.host_port.as_ref().ok_or(Errors::UnExpectedUnwrap)?,
            )
            .await?;

        match conn {
            NetConn::TCP(mut outbound) => {
                let (x, y) = tokio::io::copy_bidirectional(&mut inbound, &mut outbound).await?;
                Ok(())
            }
            _ => {
                return Err(Errors::InvalidConnectionType);
            }
        }
    }

    fn handle_udp_packet(&self, socket: &dyn UDPPacket, metadata: Box<Metadata>) {
        todo!()
    }

    fn handle_error(&self, err: Error) {
        todo!()
    }
}

impl Tunnel {
    fn resolve_metadata(&self, meta: &Metadata) -> Box<dyn ProxyAdapter> {
        Box::new(outbound::direct::Direct::new())
    }
}
