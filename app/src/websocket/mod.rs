use {
    common::api::Request,
    leptos::{create_effect, Scope},
};

mod provider;
mod request;

/// Provides a websocket to the scope.
#[allow(unused_variables)]
pub fn provide(cx: Scope) {
    #[cfg(feature = "hydrate")]
    provider::provide(cx);
}

/// Make an api request.
#[allow(unused_variables)]
pub fn request<R>(cx: Scope, request: R)
where
    R: Request + 'static,
{
    #[cfg(feature = "hydrate")]
    request::request(cx, request);
}
