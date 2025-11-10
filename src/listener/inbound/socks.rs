use crate::constant::{InboundListener, Tunnel};

use crate::constant::metadata::{Metadata, MetadataBuilder, Network, SourceType};
use crate::errors::Errors;
use async_trait::async_trait;
use fast_socks5::util::target_addr::TargetAddr;
use fast_socks5::{
    ReplyError, Result, Socks5Command, SocksError,
    server::{DnsResolveHelper as _, Socks5ServerProtocol, run_tcp_proxy, run_udp_proxy},
};
use std::net::SocketAddr;
use tokio::net::{TcpListener, TcpStream};

pub struct Socks {
}

impl Socks {
    pub fn new() -> Self {
        Socks {}
    }
}

#[async_trait]
impl InboundListener for Socks {
    fn name(&self) -> &str {
        "socks"
    }

    async fn listen(&self, tunnel: &dyn Tunnel) -> Result<(), Errors> {
        let listener = TcpListener::bind("0.0.0.0:7892").await?;

        loop {
            match listener.accept().await {
                Ok((socket, client_addr)) => {
                    self.accept(socket, client_addr, tunnel).await;
                }
                Err(err) => tunnel.handle_error(err),
            }
        }
        Ok(())
    }
}

impl Socks {
    fn read_metadata(&self, target: &TargetAddr) -> Box<Metadata> {
        let mut metadata = Box::new(Metadata::default());
        match target {
            TargetAddr::Ip(socket_addr) => {
                metadata.destination_ip = Some(socket_addr.ip());
                metadata.destination_port = socket_addr.port();
            }
            TargetAddr::Domain(domain, port) => {
                metadata.host = domain.clone();
                metadata.destination_port = port.clone();
            }
        }
        metadata.unwrap_ip();
        metadata
    }
    async fn accept(
        &self,
        socket: TcpStream,
        client_addr: SocketAddr,
        tunnel: &dyn Tunnel,
    ) -> Result<(), Errors> {
        let local_addr = socket.local_addr()?.clone();
        let s = Socks5ServerProtocol::accept_no_auth(socket).await?;
        let (proto, cmd, target) = s.read_command().await?;
        let mut meta = self.read_metadata(&target);
        meta.r#type = SourceType::SOCKS5;

        match cmd {
            Socks5Command::TCPConnect => {
                meta.network = Network::TCP;
                Ok(())
            }
            _ => Err(Errors::UnSupportedSocks5Command(cmd)),
        }?;

        let tcp_stream = proto.reply_success(local_addr).await?;
        Ok(tunnel.handle_tcp_conn(tcp_stream, meta).await?)
    }
}

