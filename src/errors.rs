use std::io;
use fast_socks5::server::SocksServerError;
use fast_socks5::Socks5Command;

#[derive(Debug)]
pub enum Errors {
    IO(io::Error),
    InvalidIpVersion,
    InvalidAddress,
    InvalidNetwork,
    Others(String),
    NotImplemented,
    UnSupportedSocks5Command(Socks5Command),
    SocksServerError(SocksServerError),
    UnExpectedUnwrap,
    InvalidConnectionType
}

impl From<io::Error> for Errors {
    fn from(err: io::Error) -> Self {
        Errors::IO(err)
    }
}

impl From<SocksServerError> for Errors {
    fn from(err: SocksServerError) -> Self {
        Errors::SocksServerError(err)
    }
}