//! This create defined the api for the backend.

use hyper::http::Method;

pub mod account;

/// The RESTful api trait.
pub trait RestApi {
    /// The request type.
    type Request;

    /// The http method to use.
    const METHOD: Method;

    /// The url path for the api resource.
    const PATH: &'static str;
}
