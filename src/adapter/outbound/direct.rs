use async_trait::async_trait;
use crate::component::dialer::{Options, WithOptions, dial_context};
use crate::constant::{AdapterType, ProxyAdapter, context, Conn, Transport, HostPort};
use crate::errors::Errors;

pub struct Direct {

}

impl Direct {
    pub fn new() -> Self {
        Direct {}
    }
}

#[async_trait]
impl ProxyAdapter for Direct {
    fn name(&self) -> &str {
        "DIRECT"
    }

    fn adapter_type(&self) -> AdapterType {
        AdapterType::Direct
    }

    fn addr(&self) -> &str {
         "UNKNOWN"
    }

    async fn dial_context(&self, ctx: &context::Context, network: Transport, addr: &HostPort) -> Result<Conn, Errors> {
        Ok(dial_context(ctx, network, addr, self.dial_options()).await?)
    }

    fn dial_options(&self) -> Vec<WithOptions> {
        vec![]
    }
}