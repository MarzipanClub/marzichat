//! The websocket module.

use {
    common::api::{AppMessage, BackendMessage},
    futures::{
        channel::mpsc::{UnboundedReceiver, UnboundedSender},
        lock::Mutex,
        SinkExt, StreamExt,
    },
    gloo::{
        net::{websocket, websocket::futures::WebSocket},
        timers::callback::Interval,
    },
    leptos::Scope,
    wasm_bindgen::UnwrapThrowExt,
};

/// The websocket connection to the backend.
pub struct Websocket {
    value: String,
}

/// Creates a new websocket.
pub fn provide_websocket(cx: Scope) {
    // cx.provide(Context {
    //     value: String::from("Hello, world!"),
    // });
}

fn create_websocket() {
    // Note that `open()` will succeed unless the url is malformed and even if
    // it cannot connect to the server.
    let (mut write, mut read) = WebSocket::open(common::routes::WEBSOCKET_URL)
        .unwrap_throw()
        .split();

    // create an mpsc channel for app components to send messages to the backend
    let (sender, mut receiver): (_, UnboundedReceiver<AppMessage>) =
        futures::channel::mpsc::unbounded();

    // send all channel messages to the backend
    {
        // let mut sender = sender.clone();
        // let reconnect = reconnect.clone();
        leptos::spawn_local(async move {
            // send an initial ping to the backend
            if write
                .send(gloo::net::websocket::Message::Bytes(
                    bincode::serialize(&AppMessage::Ping).unwrap_throw(),
                ))
                .await
                .is_err()
            {
                log::error!("failed to connect to server");
                // reconnect.toggle();
            };
            log::debug!("ping");

            // // send all the items in queue to sender.
            // if let Some(queue_receiver) = queued_messages {
            //     for message in queue_receiver.lock().await.iter() {
            //         if sender.send(message.clone()).await.is_err() {
            //             log::error!("failed to send queued message");
            //             reconnect.toggle();
            //         }
            //     }
            // }

            // loop sending items from channel to backend
            while let Some(message) = receiver.next().await {
                log::info!("<- {message:#?}");
                match bincode::serialize(&message) {
                    Ok(payload) => {
                        if let Err(error) = write.send(websocket::Message::Bytes(payload)).await {
                            log::error!("failed to send message to server: {error}");
                        }
                    }
                    Err(error) => {
                        log::error!("failed to serialize message: {error}");
                    }
                }
            }
        });
    }
}
