use self::super::{
    AdapterType, Context, HostPort, Metadata, NetConn, ProxyInfo, ProxyState, Transport,
};
use crate::component::dialer::{NetDialer, WithOptions};
use crate::constant::DNSPrefer;
use crate::constant::metadata::DialerNetwork;
use crate::constant::types::DelayHistory;
use crate::errors::Errors;
use async_trait::async_trait;
use serde::Serialize;

#[async_trait]
pub trait ProxyAdapter: Send + Sync {
    fn name(&self) -> &str;
    fn r#type(&self) -> AdapterType;
    fn addr(&self) -> &str;
    fn support_udp(&self) -> bool;
    fn proxy_info(&self, ret: &mut ProxyInfo);
    async fn dial_context(
        &self,
        ctx: &Context,
        network: Transport,
        addr: &HostPort,
    ) -> Result<NetConn, Errors>;
    fn listen_packet_context(&self, ctx: &Context, meta: &Metadata) -> Result<&dyn PacketConn, Errors>;

    fn support_uot(&self) -> bool;
    fn support_with_dialer(&self) -> DialerNetwork;


    async fn dial_context_with_dialer(
        &self,
        ctx: &Context,
        dialer: &dyn NetDialer,
        network: Transport,
        addr: &HostPort,
    ) -> Result<NetConn, Errors>;


    fn listen_packet_with_dialer(&self, ctx: &Context, dialer: &dyn NetDialer, meta: &Metadata) -> Result<&dyn PacketConn, Errors>;

    fn is_l3_protocol(&self, meta: &Metadata) -> bool;

    fn unwrap(&self) -> &dyn Proxy;

    async fn close(&self) -> Result<(), Errors>;

    fn dial_options(&self) -> Vec<WithOptions>;

    fn resolve_udp(&self, ctx: &Context, meta: &mut Metadata);
}

pub trait Proxy: ProxyAdapter {
    fn alive_for_test_url(&self, url: &str) -> bool;
    fn delay_history(&self) -> &[DelayHistory];
    fn extra_delay_histories(&self, key: &str) -> &ProxyState;
    fn last_delay_for_test_url(&self, url: &str) -> u16;
    fn url_test(&self, ctx: &Context, url: &str, expect_status: Vec<u16>) -> Result<u16, Errors>;
    fn dial(&self, meta: &Metadata) -> Result<NetConn, Errors>;
    fn dial_udp(&self, meta: &Metadata) -> Result<NetConn, Errors>;
}

#[derive(Debug, Default, Serialize)]
pub struct Base {
    pub name: String,
    pub addr: String,
    pub interface: String,
    pub r#type: AdapterType,
    pub udp: bool,
    pub xudp: bool,
    pub tfo: bool,
    pub mp_tcp: bool,
    pub routing_mark: i32,
    pub id: String,
    pub prefer: DNSPrefer,
}

pub trait HasBase {
    fn base(&self) -> &Base;
}

#[async_trait]
impl ProxyAdapter for Base {
    fn name(&self) -> &str {
        &self.name
    }

    fn r#type(&self) -> AdapterType {
        self.r#type
    }

    fn addr(&self) -> &str {
        &self.addr
    }

    fn support_udp(&self) -> bool {
        self.udp
    }

    fn proxy_info(&self, ret: &mut ProxyInfo) {
        ret.xudp = self.xudp;
        ret.tfo = self.tfo;
        ret.mp_tcp = self.mp_tcp;
        ret.smux = false;
        ret.interface = self.interface.clone();
        ret.routing_mark = self.routing_mark;
    }

    async fn dial_context(
        &self,
        ctx: &Context,
        network: Transport,
        addr: &HostPort,
    ) -> Result<NetConn, Errors> {
        todo!()
    }

    fn listen_packet_context(&self, ctx: &Context, meta: &Metadata) -> Result<&dyn PacketConn, Errors> {
        Err(Errors::NotImplemented)
    }

    fn support_uot(&self) -> bool {
        false
    }

    fn support_with_dialer(&self) -> DialerNetwork {
        todo!()
    }

    async fn dial_context_with_dialer(&self, ctx: &Context, dialer: &dyn NetDialer, network: Transport, addr: &HostPort) -> Result<NetConn, Errors> {
        Err(Errors::NotImplemented)
    }

    fn listen_packet_with_dialer(&self, ctx: &Context, dialer: &dyn NetDialer, meta: &Metadata) -> Result<&dyn PacketConn, Errors> {
        Err(Errors::NotImplemented)
    }

    fn is_l3_protocol(&self, meta: &Metadata) -> bool {
        todo!()
    }

    fn unwrap(&self) -> &dyn Proxy {
        todo!()
    }

    async fn close(&self) -> Result<(), Errors> {
        todo!()
    }

    fn dial_options(&self) -> Vec<WithOptions> {
        vec![
            WithOptions::InterfaceName(self.interface.clone()),
            WithOptions::RoutingMark(self.routing_mark),
            match self.prefer {
                DNSPrefer::IpV4Only => WithOptions::Network(4),
                DNSPrefer::IpV6Only => WithOptions::Network(6),
                DNSPrefer::IpV4Prefer => WithOptions::Prefer(4),
                DNSPrefer::IpV6Prefer => WithOptions::Prefer(6),
                _ => WithOptions::Nothing
            },
            WithOptions::TFO(self.tfo),
            WithOptions::TFO(self.mp_tcp),
        ]
    }

    fn resolve_udp(&self, ctx: &Context, meta: &mut Metadata) {
        todo!()
    }
}

pub trait PacketConn {
    fn resolve_udp(&self, ctx: &Context, meta: &mut Metadata);
}
