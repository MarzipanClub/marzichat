use {
    actix_governor::{
        governor::{
            clock::{Clock, DefaultClock, QuantaInstant},
            NotUntil,
        },
        KeyExtractor, SimpleKeyExtractionError,
    },
    actix_web::{
        dev::ServiceRequest,
        http::{header::ContentType, StatusCode},
        HttpRequest, HttpResponse, HttpResponseBuilder,
    },
    std::net::IpAddr,
};

/// The error type for the client ip address parser.
#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// The client ip address was not provided in the request.
    #[error("Missing client ip address.")]
    MissingClientIpAddress,

    /// The client ip address could not be parsed.
    #[error("Error parsing the client ip address.")]
    InvalidClientIpAddress,
}

/// Parse the client ip address from the request.
pub fn parse(request: &HttpRequest) -> Result<IpAddr, Error> {
    request
        .connection_info()
        .peer_addr()
        .ok_or(Error::MissingClientIpAddress)?
        .parse()
        .map_err(|_| Error::InvalidClientIpAddress)
}

/// The ip address extractor.
#[derive(Clone)]
pub struct Extractor;

impl KeyExtractor for Extractor {
    type Key = IpAddr;
    type KeyExtractionError = SimpleKeyExtractionError<&'static str>;

    fn extract(&self, request: &ServiceRequest) -> Result<Self::Key, Self::KeyExtractionError> {
        parse(request.request()).map_err(|_| {
            Self::KeyExtractionError::new("Bad Request: failed to parse ip address")
                .set_content_type(ContentType::plaintext())
                .set_status_code(StatusCode::BAD_REQUEST)
        })
    }

    fn exceed_rate_limit_response(
        &self,
        negative: &NotUntil<QuantaInstant>,
        mut response: HttpResponseBuilder,
    ) -> HttpResponse {
        let wait_time = negative
            .wait_time_from(DefaultClock::default().now())
            .as_secs();

        response
            .status(StatusCode::TOO_MANY_REQUESTS)
            .content_type(ContentType::plaintext())
            .append_header((actix_web::http::header::RETRY_AFTER, wait_time))
            .body(format!(
                "Too Many Requests: retry after {wait_time} seconds"
            ))
    }
}
