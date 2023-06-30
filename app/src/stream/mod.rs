//! Module for websocket and stream related code.

mod backoff;
mod connection;
mod provider;
mod request;

pub use {
    provider::provide_connection,
    request::{request, request_deferred, RequestState},
};
