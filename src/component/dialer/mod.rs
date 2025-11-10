use crate::constant::context::Context;
use crate::constant::{Conn, HostPort, Transport, context};
use crate::errors::Errors;
use derive_builder::Builder;
use paste::paste;
use std::net;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use tokio::net::{TcpStream, UdpSocket};


#[derive(Builder, Clone, Default)]
pub struct Options {
    pub interface_name: Option<String>,
    pub fallback_bind: Option<bool>,
    pub addr_reuse: Option<bool>,
    pub routing_mark: Option<i32>,
    pub network: i32,
    pub prefer: Option<i32>,
    pub tfo: Option<bool>,
    pub mp_tcp: Option<bool>,
}

pub enum WithOptions {
    InterfaceName(Option<String>),
    FallbackBind(Option<bool>),
    AddrReuse(Option<bool>),
    RoutingMark(Option<i32>),
    Network(i32),
    Prefer(Option<i32>),
    TFO(Option<bool>),
    MPTCP(Option<bool>),
}

impl WithOptions {
    pub fn apply(self, opts: &mut Options) {
        match self {
            WithOptions::InterfaceName(val) => opts.interface_name = val,
            WithOptions::FallbackBind(val) => opts.fallback_bind = val,
            WithOptions::AddrReuse(val) => opts.addr_reuse = val,
            WithOptions::RoutingMark(val) => opts.routing_mark = val,
            WithOptions::Network(val) => opts.network = val,
            WithOptions::Prefer(val) => opts.prefer = val,
            WithOptions::TFO(val) => opts.tfo = val,
            WithOptions::MPTCP(val) => opts.mp_tcp = val,
        }
    }
}


fn apply_options(options: Vec<WithOptions>) -> Options {
    let mut opts = Options::default();
    for option in options {
        option.apply(&mut opts);
    }
    opts
}

pub async fn dial_context(
    ctx: &context::Context,
    mut network: Transport,
    addr: &HostPort,
    options: Vec<WithOptions>,
) -> Result<Conn, Errors> {
    let opts = apply_options(options);
    network = network.normalized(opts.network)?;

    match network {
        Transport::TCP4 | Transport::TCP6 | Transport::UDP4 | Transport::UDP6 => {
            Ok(actual_single_stack_dial_context(ctx, network, addr, &opts).await?)
        }
        _ => Ok(actual_single_stack_dial_context(ctx, network, addr, &opts).await?),
    }
}

async fn actual_single_stack_dial_context(
    ctx: &context::Context,
    network: Transport,
    addr: &HostPort,
    options: &Options,
) -> Result<Conn, Errors> {
    match addr {
        HostPort::IP(ip, port) => dial_context_internal(
            ctx, network, ip.clone(), port.clone(), options
        ).await,
        HostPort::Domain(_, _) => Err(Errors::NotImplemented),
    }
}

async fn actual_dual_stack_dial_context(
    ctx: &context::Context,
    network: Transport,
    addr: &HostPort,
    options: &Options,
) -> Result<Conn, Errors> {
    Err(Errors::NotImplemented)
}

pub trait NetDialer {
    fn dial_context(
        &self,
        ctx: context::Context,
        network: &str,
        addr: &str,
    ) -> Result<Conn, crate::errors::Errors>;
}

pub struct SystemDialer {}

impl SystemDialer {
    async fn dial_context_tcp(
        &self,
        ctx: Context,
        network: &str,
        addr: &str,
    ) -> Result<TcpStream, Errors> {
        let stream = TcpStream::connect(addr).await?;
        Ok(stream)
    }
}

impl NetDialer for SystemDialer {
    fn dial_context(&self, ctx: Context, network: &str, addr: &str) -> Result<Conn, Errors> {
        todo!()
    }
}

async fn dial_context_internal(
    ctx: &context::Context,
    network: Transport,
    addr: net::IpAddr,
    port: u16,
    options: &Options,
) -> Result<Conn, Errors> {
    match network {
        Transport::TCP4 | Transport::TCP6 | Transport::TCP => {
            let conn = TcpStream::connect((addr, port)).await?;
            Ok(Conn::TCP(conn))
        }
        Transport::UDP | Transport::UDP4 | Transport::UDP6 => {
            let zero_addr: IpAddr = if addr.is_ipv4() {
                Ipv4Addr::from_bits(0).into()
            } else {
                Ipv6Addr::from_bits(0).into()
            };
            let udp = UdpSocket::bind((zero_addr, 0)).await?;
            udp.connect((addr, port)).await?;
            Ok(Conn::UDP(udp))
        }
    }
}
