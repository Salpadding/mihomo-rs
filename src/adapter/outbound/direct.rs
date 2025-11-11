use async_trait::async_trait;
use crate::component::dialer::{NetDialer, WithOptions};
use crate::constant::{AdapterType, Proxy, context, NetConn, Transport, HostPort, ProxyAdapter, ProxyInfo, Context, Metadata};
use crate::constant::adapters::{Base, HasBase, PacketConn};
use crate::constant::metadata::DialerNetwork;
use crate::errors::Errors;

pub struct Direct {
    base: Base,
}

impl HasBase for Direct {
    fn base(&self) -> &Base {
        &self.base
    }
}

impl Direct {
    pub fn new() -> Self {
        Direct {
            base: Base::default(),
        }
    }
}

#[async_trait]
impl ProxyAdapter for Direct {
    fn name(&self) -> &str {
        &self.base().name
    }

    fn r#type(&self) -> AdapterType {
        self.base().r#type
    }

    fn addr(&self) -> &str {
        &self.base().addr
    }

    fn support_udp(&self) -> bool {
        self.base().udp
    }

    fn proxy_info(&self, ret: &mut ProxyInfo) {
        self.base().proxy_info(ret);
    }

    async fn dial_context(&self, ctx: &Context, network: Transport, addr: &HostPort) -> Result<NetConn, Errors> {
        self.base.dial_context(ctx, network, addr).await
    }

    fn listen_packet_context(&self, ctx: &Context, meta: &Metadata) -> Result<&dyn PacketConn, Errors>{
        self.base.listen_packet_context(ctx, meta)
    }

    fn support_uot(&self) -> bool {
        self.base.support_uot()
    }

    fn support_with_dialer(&self) -> DialerNetwork {
        self.base.support_with_dialer()
    }

    async fn dial_context_with_dialer(&self, ctx: &Context, dialer: &dyn NetDialer, network: Transport, addr: &HostPort) -> Result<NetConn, Errors> {
        Ok(self.base.dial_context_with_dialer(ctx, dialer, network, addr).await?)
    }

    fn listen_packet_with_dialer(&self, ctx: &Context, dialer: &dyn NetDialer, meta: &Metadata) -> Result<&dyn PacketConn, Errors> {
        todo!()
    }

    fn is_l3_protocol(&self, meta: &Metadata) -> bool {
        self.base.is_l3_protocol(meta)
    }

    fn unwrap(&self) -> &dyn Proxy {
        todo!()
    }

    async fn close(&self) -> Result<(), Errors> {
        todo!()
    }

    fn dial_options(&self) -> Vec<WithOptions> {
        self.base.dial_options()
    }

    fn resolve_udp(&self, ctx: &Context, meta: &mut Metadata) {
        self.base.resolve_udp(ctx, meta);
    }
}

