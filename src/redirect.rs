//! Middleware for redirecting requests to https.
#![cfg(feature = "ssr")]

use {
    actix_web::{
        body::EitherBody,
        dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
        http, Error, HttpResponse,
    },
    futures_util::future::{ok, LocalBoxFuture, Ready},
};

/// Middleware to redirect http requests to https if the connection is not
/// already secure. Enabled only if tls is configured.
pub struct HttpToHttps;

impl<S, B> Transform<S, ServiceRequest> for HttpToHttps
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Error = Error;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;
    type InitError = ();
    type Response = ServiceResponse<EitherBody<B>>;
    type Transform = RedirectMiddleware<S>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(RedirectMiddleware { service })
    }
}

pub struct RedirectMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for RedirectMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;
    type Response = ServiceResponse<EitherBody<B>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        if req.connection_info().scheme() == "https" || cfg!(debug_assertions) {
            let res = self.service.call(req);

            Box::pin(async move { res.await.map(ServiceResponse::map_into_left_body) })
        } else {
            let host = req.connection_info().host().to_owned();
            let uri = req.uri().to_owned();
            let url = format!("https://{host}{uri}");

            let response = HttpResponse::MovedPermanently()
                .append_header((http::header::LOCATION, url))
                .finish()
                .map_into_right_body();

            Box::pin(async { Ok(req.into_response(response)) })
        }
    }
}
