#![cfg(feature = "hydrate")]
//! Websocket and associated utilities.

use {
    common::api::{AppMessage, BackendMessage},
    futures::{channel::mpsc::UnboundedReceiver, SinkExt, StreamExt},
    gloo::net::websocket::{futures::WebSocket, Message},
    leptos::{provide_context, Scope},
    wasm_bindgen::UnwrapThrowExt,
};

pub fn provide(cx: Scope) {
    let (mut write, mut read) = WebSocket::open(common::routes::WEBSOCKET_URL)
        .unwrap_throw()
        .split();

    // create an mpsc channel for components to send messages into the websocket
    let (sender, mut receiver): (_, UnboundedReceiver<AppMessage>) =
        futures::channel::mpsc::unbounded();

    // loop sending items from channel to backend
    leptos::spawn_local(async move {
        while let Some(message) = receiver.next().await {
            match bincode::serialize(&message) {
                Ok(payload) => {
                    if let Err(error) = write.send(Message::Bytes(payload)).await {
                        leptos::error!("failed to send message to backend: {error}");
                    }
                }
                Err(error) => {
                    leptos::error!("failed to serialize message: {error}");
                }
            }
        }
    });

    // loop read messages from the backend
    leptos::spawn_local(async move {
        while let Some(Ok(Message::Bytes(payload))) = read.next().await {
            match bincode::deserialize::<BackendMessage>(&payload) {
                Ok(message) => {
                    // TODO: handle server message
                    todo!("handle server message: {message:#?}");
                }
                Err(error) => {
                    leptos::error!("failed to deserialize server message: {error}");
                }
            }
        }
    });

    provide_context(cx, sender);
}
