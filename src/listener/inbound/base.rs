use std::net;
use std::net::{IpAddr, Ipv4Addr};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use crate::common::ranges;
use crate::common::ranges::{IntRanges, IntRangesExt};
use crate::constant::{HostPort, InboundListener};
use crate::constant::listener::InboundConfig;
use crate::errors::Errors;

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct BaseOption {
    pub name: String,
    pub listen: String,
    pub port: String,
    pub special_rules: String,
    pub special_proxy: String
}


#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Base {
    config: BaseOption,
    name: String,
    special_rules: String,
    listen_addr: Option<net::IpAddr>,
    ports: ranges::IntRanges<u16>
}

impl Base {
    pub fn new(mut options: BaseOption) -> Result<Self, Errors> {
        if options.listen.is_empty() {
            options.listen = "0.0.0.0".to_string();
        } 
        let addr = IpAddr::parse_ascii(options.listen.as_bytes())?;
        let ports: IntRanges<u16> = ranges::new_unsigned_ranges(&options.port)?;
        
        Ok(Base {
            name: options.name.clone(),
            listen_addr: Some(addr),
            special_rules: options.special_rules.clone(),
            ports,
            config: options,
        })
    }
}

impl BaseOption {
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl InboundConfig for BaseOption {
    fn name(&self) -> &str {
        &self.name
    }
}

#[async_trait]
impl InboundListener for Base {
    fn name(&self) -> &str {
        &self.name
    }

    async fn listen(&self, tunnel: &dyn crate::constant::Tunnel) -> Result<(), crate::errors::Errors> {
        Ok(())
    }

    fn address(&self) -> String {
        self.raw_address()
    }

    fn raw_address(&self) -> String {
        let ip = self.listen_addr.unwrap_or(IpAddr::V4(Ipv4Addr::UNSPECIFIED));
        if self.ports.is_empty() {
            return HostPort::Ip(ip, 0).to_string()
        }
        let mut addresses: Vec<HostPort> = Vec::new();
        self.ports.range(|p| {
            addresses.push(HostPort::Ip(ip, p));
            true
        });
        addresses.into_iter().
            map(|a| a.to_string()).
            collect::<Vec<String>>().join(",")
    }

    fn config(&self) -> &dyn InboundConfig {
        &self.config
    }
}