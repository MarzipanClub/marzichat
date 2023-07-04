use {common::api::AppMessage, leptos::Scope};

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
pub fn request(cx: Scope, message: AppMessage) {
    #[cfg(feature = "hydrate")]
    request::request(cx, message);
}
