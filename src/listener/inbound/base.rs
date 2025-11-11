use std::net;

pub struct BaseOption {
    pub name: String,
    pub listen: String,
    pub port: String,
    pub special_rules: String,
    pub special_proxy: String
}

pub struct Base {
    config: BaseOption,
    name: String,
    special_rules: String,
    listen_addr: net::IpAddr,
}