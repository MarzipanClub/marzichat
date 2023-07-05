#![cfg(feature = "hydrate")]
//! Send request api calls through a websocket connection.

use {
    common::api::{AppMessage, Request},
    futures::{channel::mpsc::UnboundedSender, SinkExt},
    leptos::{expect_context, spawn_local, Scope},
    wasm_bindgen::UnwrapThrowExt,
};

pub enum State<T> {
    Pending,
    Data(T),
    Failed,
}

pub fn request<R>(cx: Scope, request: R)
where
    R: Request + 'static,
{
    let sender = expect_context::<UnboundedSender<AppMessage>>(cx);
    leptos::log!("sending request");

    let app_message: AppMessage = request.into();
    leptos::log!("sending request: 2 {:#?}", app_message);
    sender
        .unbounded_send(app_message)
        .expect_throw("failed to send message");
}
