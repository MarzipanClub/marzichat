//! The websocket module.

use {
    common::api::{AppMessage, BackendMessage, HEARTBEAT_INTERVAL},
    futures::{
        channel::mpsc::{UnboundedReceiver, UnboundedSender},
        lock::Mutex,
        SinkExt, StreamExt,
    },
    gloo::{
        net::{websocket, websocket::futures::WebSocket},
        timers::callback::Interval,
    },
    leptos::Trigger,
    std::rc::Rc,
    wasm_bindgen::UnwrapThrowExt,
};

/// The various errors that can occur when sending a message through the
/// websocket.
pub enum Error {
    // An error representing that the websocket is not connected or connected but
    // not ready to be used. A message was queued until the websocket is ready.
    Queued,

    // Failed to place the message in the channel.
    SendFailed,
}

/// The state of the connection.
enum State {
    /// The initial state which just queues messages until a connection can be
    /// made.
    Uninitialized { queue: Rc<Mutex<Vec<AppMessage>>> },

    /// A perhaps ready state with a transmitter for sending messages to the
    /// server. May not be fully functional until the server's pong is
    /// received.
    MaybeReady {
        sender: UnboundedSender<AppMessage>,
        reconnect_now: Trigger,
        heartbeat: Interval,
    },
}

/// The websocket connection to the backend.
pub struct Connection(State);

impl Connection {
    /// Creates an uninitialized connection.
    pub fn uninitialized() -> Self {
        Self(State::Uninitialized {
            queue: Rc::new(Mutex::new(Vec::new())),
        })
    }

    /// Creates a new websocket connection to the server.
    /// Sends out an initial ping and toggles the `is_ready` when ready.
    ///
    /// Toggles the `reconnect` when the connection is closed or
    /// lost.
    /// Toggles the `reconnect_now` when the `.send()` method is called
    pub fn new(
        reconnect: Trigger,
        reconnect_now: Trigger,
        is_ready: Trigger,
        queued_messages: Option<Rc<Mutex<Vec<AppMessage>>>>,
    ) -> Self {
        log::info!("initializing websocket connection");

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
            let mut sender = sender.clone();
            log::debug!("sending initial ping");
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
                    reconnect.notify();
                } else {
                    log::debug!("ping");
                }

                // send all the items in queue to sender.
                if let Some(queue_receiver) = queued_messages {
                    for message in queue_receiver.lock().await.iter() {
                        if sender.send(message.clone()).await.is_err() {
                            log::error!("failed to send queued message");
                            reconnect.track();
                        }
                    }
                }

                // loop sending items from channel to backend
                while let Some(message) = receiver.next().await {
                    if AppMessage::Heartbeat != message {
                        log::info!("→ {message:#?}");
                    }
                    match bincode::serialize(&message) {
                        Ok(payload) => {
                            if let Err(error) = write.send(websocket::Message::Bytes(payload)).await
                            {
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

        // read messages from the server send them to the store
        leptos::spawn_local(async move {
            while let Some(Ok(websocket::Message::Bytes(payload))) = read.next().await {
                match bincode::deserialize(&payload) {
                    Ok(message @ BackendMessage::Pong) => {
                        log::info!("← {message:#?}");
                        is_ready.notify();
                    }
                    Ok(message) => {
                        log::info!("← {message:#?}");
                        // crate::stream::store::handle_server_message(message);
                        todo!("handle server message");
                    }
                    Err(error) => {
                        log::error!("failed to deserialize server message: {error}");
                    }
                }
            }
            reconnect.notify();
        });

        let heartbeat = {
            let sender = sender.clone();
            log::debug!("sending heartbeat every {HEARTBEAT_INTERVAL:?}");
            Interval::new(HEARTBEAT_INTERVAL.as_millis() as _, move || {
                let mut sender = sender.clone();
                leptos::spawn_local(async move {
                    if sender.send(AppMessage::Heartbeat).await.is_err() {
                        log::error!("failed to send heartbeat");
                    }
                });
            })
        };

        Self(State::MaybeReady {
            sender,
            reconnect_now,
            heartbeat,
        })
    }

    /// Returns the mutex of the queue of messages.
    pub fn get_queued(&self) -> Option<Rc<Mutex<Vec<AppMessage>>>> {
        match self.0 {
            State::Uninitialized { ref queue } => Some(queue.clone()),
            State::MaybeReady { .. } => None,
        }
    }

    /// Sends a message to the server. Returns an error if the websocket is not
    /// ready.
    ///
    /// Note: The message is not guaranteed to be sent even if no error is
    /// returned.
    pub async fn send(&self, message: AppMessage) -> Result<(), Error> {
        match self.0 {
            State::MaybeReady {
                ref sender,
                ref reconnect_now,
                ..
            } => match sender.clone().send(message).await {
                Ok(()) => Ok(()),
                Err(_) => {
                    reconnect_now.notify();
                    Err(Error::SendFailed)
                }
            },
            State::Uninitialized { ref queue } => {
                queue.lock().await.push(message);
                Err(Error::Queued)
            }
        }
    }
}
