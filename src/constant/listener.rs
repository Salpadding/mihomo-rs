use async_trait::async_trait;
use crate::errors::Errors;
use self::super::Tunnel;

#[async_trait]
pub trait InboundListener {
    fn name(&self) -> &str;
    async fn listen(&self, tunnel: &dyn Tunnel) -> Result<(), Errors>;
}

