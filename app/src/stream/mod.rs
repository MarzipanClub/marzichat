//! Module for websocket and stream related code.

mod backoff;
pub mod connection; // todo make private
pub mod provider; // todo make private
mod request;

pub use {
    provider::provide_connection,
    request::{request, request_deferred, RequestState},
};
