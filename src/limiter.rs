//! Rate limiter based on actix governor.
#![cfg(feature = "ssr")]

use {
    crate::{address, config::RateLimiterConfig},
    actix_governor::{
        governor::{
            clock::{Clock, DefaultClock, QuantaInstant},
            middleware::StateInformationMiddleware,
            NotUntil,
        },
        Governor, GovernorConfigBuilder, KeyExtractor, SimpleKeyExtractionError,
    },
    actix_web::{
        dev::ServiceRequest,
        http::{header::ContentType, StatusCode},
        HttpResponse, HttpResponseBuilder,
    },
    std::net::IpAddr,
};

/// Returns a new rate limiter governor layer
pub fn layer(
    config: RateLimiterConfig,
) -> Governor<IpAddressExtractor, StateInformationMiddleware> {
    Governor::new(
        &GovernorConfigBuilder::default()
            .per_second(config.replenish_interval_seconds.get())
            .burst_size(config.burst_size.get())
            .key_extractor(IpAddressExtractor)
            .use_headers()
            .finish()
            .expect("invalid governor configuration"),
    )
}

/// The ip address extractor used for rate limiting.
#[derive(Clone)]
pub struct IpAddressExtractor;

impl KeyExtractor for IpAddressExtractor {
    type Key = IpAddr;
    type KeyExtractionError = SimpleKeyExtractionError<&'static str>;

    fn extract(&self, request: &ServiceRequest) -> Result<Self::Key, Self::KeyExtractionError> {
        address::parse(request.request()).map_err(|_| {
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
