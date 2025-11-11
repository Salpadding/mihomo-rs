use std::io;
use std::net::AddrParseError;
use fast_socks5::server::SocksServerError;
use fast_socks5::Socks5Command;
use crate::common::ranges::IntRangesError;

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
    InvalidConnectionType,
    AddrParseError(AddrParseError),
    IntRangesError(IntRangesError)
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

impl From<AddrParseError> for Errors {
    fn from(err: AddrParseError) -> Self {
        Errors::AddrParseError(err)
    }
}

impl From<IntRangesError> for Errors {
    fn from(err: IntRangesError) -> Self {
        Errors::IntRangesError(err)
    }
}