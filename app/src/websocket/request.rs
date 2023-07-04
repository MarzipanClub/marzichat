#![cfg(feature = "hydrate")]
//! Send request api calls through a websocket connection.

use {
    common::api::AppMessage,
    futures::{channel::mpsc::UnboundedSender, SinkExt},
    leptos::{use_context, Scope},
    wasm_bindgen::UnwrapThrowExt,
};

pub fn request(cx: Scope, message: AppMessage) {
    let mut sender =
        use_context::<UnboundedSender<AppMessage>>(cx).expect_throw("no message sender found");

    leptos::spawn_local(async move {
        sender
            .send(message)
            .await
            .expect_throw("failed to send message");
    });
}
