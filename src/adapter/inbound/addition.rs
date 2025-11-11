use std::net;
use crate::constant::dns::DNSMode;
use crate::constant::Metadata;
use crate::constant::metadata::{DialerNetwork, SourceType};

#[derive(Debug, Clone)]
pub enum Addition {
    Network(DialerNetwork),
    Type(SourceType),
    SourceIp(net::IpAddr),
    DestinationIp(net::IpAddr),

    SourceGeoIp(Vec<String>),
    DestinationGeoIp(Vec<String>),
    SourceIpAsn(String),
    DestinationIpAsn(String),

    SourcePort(u16),
    DestinationPort(u16),

    InboundIp(net::IpAddr),
    InboundPort(u16),
    InboundName(String),
    InboundUser(String),

    Host(String),
    DNSMode(DNSMode),
    Uid(u32),
    Process(String),
    ProcessPath(String),

    SpecialProxy(String),
    SpecialRules(String),
    RemoteDestination(String),

    DSCP(u8),
    SniffHost(String),
}

impl Addition {
    pub fn apply(self, metadata: &mut Metadata) {
        match self {
            Addition::Network(val) => metadata.network = val,
            Addition::Type(val) => metadata.r#type = val,
            Addition::SourceIp(ip) => metadata.source_ip = Some(ip),
            Addition::DestinationIp(ip) => metadata.destination_ip = Some(ip),

            Addition::SourceGeoIp(v) => metadata.source_geo_ip = v,
            Addition::DestinationGeoIp(v) => metadata.destination_geo_ip = v,
            Addition::SourceIpAsn(s) => metadata.source_ip_asn = s,
            Addition::DestinationIpAsn(s) => metadata.destination_ip_asn = s,

            Addition::SourcePort(p) => metadata.source_port = p,
            Addition::DestinationPort(p) => metadata.destination_port = p,

            Addition::InboundIp(ip) => metadata.inbound_ip = Some(ip),
            Addition::InboundPort(p) => metadata.inbound_port = p,
            Addition::InboundName(s) => metadata.inbound_name = s,
            Addition::InboundUser(s) => metadata.inbound_user = s,

            Addition::Host(s) => metadata.host = s,
            Addition::DNSMode(mode) => metadata.dns_mode = mode,
            Addition::Uid(uid) => metadata.uid = uid,
            Addition::Process(s) => metadata.process = s,
            Addition::ProcessPath(s) => metadata.process_path = s,

            Addition::SpecialProxy(s) => metadata.special_proxy = s,
            Addition::SpecialRules(s) => metadata.special_rules = s,
            Addition::RemoteDestination(s) => metadata.remote_destination = s,

            Addition::DSCP(d) => metadata.dscp = d,
            Addition::SniffHost(s) => metadata.sniff_host = s,
        }
    }

    pub fn apply_all(additions: Vec<Addition>, metadata: &mut Metadata) {
        for addition in additions {
            addition.apply(metadata);
        }
    }
}