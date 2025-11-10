use crate::constant::HostPort;
use crate::constant::metadata::Metadata;

pub struct Context {
    pub metadata: Box<Metadata>,
    pub host_port: Option<HostPort>
}

impl Context {
    pub fn new(meta: Box<Metadata>, host_port: HostPort) -> Context {
        Context {
           metadata: meta,
            host_port: Some(host_port),
        }
    }

}