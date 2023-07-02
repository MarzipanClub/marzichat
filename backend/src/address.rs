//! Client ip address parser.

use {
    actix_web::HttpRequest,
    std::net::{AddrParseError, IpAddr},
};

/// The error type for the client ip address parser.
#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// The client ip address was not provided in the request.
    #[error("missing client ip address")]
    MissingClientIpAddress,

    /// The client ip address could not be parsed.
    #[error("error parsing the client ip address: {0}")]
    InvalidClientIpAddress(#[from] AddrParseError),
}

/// Parse the client ip address from the request.
pub fn parse(request: &HttpRequest) -> Result<IpAddr, Error> {
    Ok(request
        .connection_info()
        .peer_addr()
        .ok_or(Error::MissingClientIpAddress)?
        .parse()?)
}
