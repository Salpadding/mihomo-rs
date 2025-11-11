use std::fmt::{Debug, Display, Pointer};
use async_trait::async_trait;
use crate::errors::Errors;
use self::super::Tunnel;

#[async_trait]
pub trait InboundListener {
    fn name(&self) -> &str;
    async fn listen(&self, tunnel: &dyn Tunnel) -> Result<(), Errors>;

    fn address(&self) -> String;
    fn raw_address(&self) -> String;
    fn config(&self) ->&dyn InboundConfig;
}

pub trait InboundConfig: Debug {
    fn name(&self) -> &str;
    fn eq(&self, other: &dyn InboundConfig) -> bool {
        format!("{:?}", self) == format!("{:?}", other)
    }
}