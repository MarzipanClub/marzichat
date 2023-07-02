use {
    crate::{config, handler},
    actix_ws::{CloseCode, CloseReason, Session},
    anyhow::Result,
    common::{
        api::{AppMessage, BackendMessage, HEARTBEAT_INTERVAL},
        types::account,
    },
    dashmap::{mapref::one::Ref, DashMap},
    std::{fmt, sync::OnceLock, time::Duration},
    tokio::{
        sync::{
            mpsc::{Receiver, Sender},
            OwnedSemaphorePermit,
        },
        task::JoinHandle,
    },
};

const ACTOR_TERMINATION_GRACE_PERIOD: Duration = Duration::from_secs(3);

#[derive(Debug)]
struct Heartbeat;

/// The id for the client.
pub type ClientId = u32;

/// The actor was terminated.
#[derive(thiserror::Error, Debug)]
pub struct Terminated(());

impl fmt::Display for Terminated {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "terminated")
    }
}

/// The various events that an actor can process.
#[derive(Debug)]
enum Event {
    /// Process an incoming webapp message.
    App(AppMessage),

    /// Send a backend message to the webapp.
    Backend(BackendMessage),

    /// Terminate the actor.
    Terminate,
}

impl From<AppMessage> for Event {
    fn from(message: AppMessage) -> Self {
        Self::App(message)
    }
}

impl From<BackendMessage> for Event {
    fn from(message: BackendMessage) -> Self {
        Self::Backend(message)
    }
}

/// The context for an actor.
pub struct Context {
    pub client_id: ClientId,
    pub authenticated_account: Option<account::Id>,
    session: Session,
    receiver: Receiver<Event>,
    heartbeat_sender: Sender<Heartbeat>,

    #[allow(dead_code)]
    permit: OwnedSemaphorePermit,
}

impl Context {
    /// Spawns an actor.
    async fn spawn(
        client_id: ClientId,
        session: Session,
        receiver: Receiver<Event>,
        authenticated_account: Option<account::Id>,
        permit: OwnedSemaphorePermit,
    ) -> Result<()> {
        let (heartbeat_sender, mut heartbeat_receiver) = tokio::sync::mpsc::channel(1);

        let mut actor = Self {
            client_id,
            session,
            receiver,
            heartbeat_sender,
            authenticated_account,
            permit,
        };

        let (close_reason, result) = match tokio::try_join!(
            run_loop(&mut actor),
            heartbeat_timeout_loop(&mut heartbeat_receiver)
        ) {
            Ok(_) => (
                CloseReason {
                    code: CloseCode::Normal,
                    description: None,
                },
                Ok(()),
            ),
            Err(error) => {
                tracing::debug!(%client_id, "{error}");
                (
                    CloseReason {
                        code: CloseCode::Error,
                        description: Some(error.to_string()),
                    },
                    Err(error),
                )
            }
        };

        // need to close the session to prevent the tcp file descriptor from
        // leaking in CLOSE_WAIT
        actor.session.close(Some(close_reason)).await?;
        tracing::debug!(%client_id, "terminated");
        remove(&client_id);
        result
    }

    /// Sends a message to the app.
    pub async fn send(&mut self, message: &BackendMessage) -> Result<()> {
        tracing::trace!(client_id = %self.client_id, ?message, "<-");
        self.session.binary(bincode::serialize(message)?).await?;
        Ok(())
    }

    /// Refreshes the heartbeat timeout.
    /// Returns an error if the heartbeat loop is full or closed.
    pub fn heartbeat(&mut self) -> Result<()> {
        self.heartbeat_sender.try_send(Heartbeat)?;
        Ok(())
    }
}

/// The main event loop for the actor.
async fn run_loop(actor: &mut Context) -> Result<()> {
    while let Some(event) = actor.receiver.recv().await {
        match event {
            Event::App(message) => {
                // skip logging heartbeat messages
                if message != AppMessage::Heartbeat {
                    tracing::trace!(client_id = %actor.client_id, ?message, "->");
                }
                handler::process(message, actor).await?
            }
            Event::Backend(message) => actor.send(&message).await?,
            Event::Terminate => {
                break;
            }
        }
    }
    Ok(())
}

/// The heartbeat loop for the actor.
/// This loop is used to know when the actor should be terminated once the
/// client stops sending heartbeats.
async fn heartbeat_timeout_loop(liveliness_receiver: &mut Receiver<Heartbeat>) -> Result<()> {
    loop {
        // will break out of the loop and return an error if the liveliness receiver
        // times out
        tokio::time::timeout(HEARTBEAT_INTERVAL, liveliness_receiver.recv())
            .await
            .map_err(|_| anyhow::anyhow!("heartbeat timeout"))?;
    }
}

/// A handle to a running actor.
pub struct ActorHandle {
    client_id: ClientId,
    handle: JoinHandle<Result<()>>,
    sender: Sender<Event>,
    pub authenticated_account: Option<account::Id>,
}

/// A sender to pass app messages to the actor.
pub struct ActorSender {
    client_id: ClientId,
    sender: Sender<Event>,
}

static ACTORS: OnceLock<DashMap<ClientId, ActorHandle>> = OnceLock::new();

/// Get the map or client ip addresses to actor.
#[inline]
fn actors() -> &'static DashMap<ClientId, ActorHandle> {
    ACTORS.get_or_init(|| DashMap::with_capacity(config::get().max_websocket_connections))
}

/// Get a reference to the actor handle.
#[inline]
pub fn get<'a>(client_id: &ClientId) -> Option<Ref<'a, ClientId, ActorHandle>> {
    actors().get(client_id)
}

/// Remove the actor.
#[inline]
pub fn remove(client_id: &ClientId) {
    actors().remove(client_id);
}

// Count of all the actors.
#[inline]
pub fn count() -> usize {
    actors().len()
}

/// Creates a new actor for the given client id.
/// The actor starts running in the background.
/// Aborts existing actor if one exists for the given client id.
pub fn spawn(
    client_id: ClientId,
    session: Session,
    authenticated_account: Option<account::Id>,
    permit: OwnedSemaphorePermit,
) -> ActorSender {
    // initialize a event channel to stream events to the actor
    let (sender, receiver) = tokio::sync::mpsc::channel(1);

    if let Some(previous_actor) = actors().insert(
        client_id,
        ActorHandle {
            client_id,
            sender: sender.clone(),
            handle: tokio::spawn(async move {
                Context::spawn(client_id, session, receiver, authenticated_account, permit).await
            }),
            authenticated_account,
        },
    ) {
        // abort any existing actor for this client
        tracing::trace!(%client_id, "aborting an existing actor");
        // try to immediately send a terminate event to the actor
        if previous_actor.sender.try_send(Event::Terminate).is_err() {
            // abort actor if its buffer is full or the channel is closed
            previous_actor.handle.abort();
        } else {
            tokio::spawn(async move {
                tokio::time::sleep(ACTOR_TERMINATION_GRACE_PERIOD).await;
                previous_actor.handle.abort();
            });
        }
    };

    ActorSender { client_id, sender }
}

impl ActorHandle {
    /// Make the actor send a backend message to the app.
    pub async fn send(&self, message: BackendMessage) -> Result<(), Terminated> {
        match self.sender.send(message.into()).await {
            Err(_) => {
                remove(&self.client_id);
                Err(Terminated(()))
            }
            Ok(_) => Ok(()),
        }
    }
}

impl ActorSender {
    /// Send an app message to the actor blocking until the actor has capacity.
    pub async fn send(&self, message: AppMessage) -> Result<(), Terminated> {
        match self.sender.send(message.into()).await {
            Err(_) => {
                remove(&self.client_id);
                Err(Terminated(()))
            }
            Ok(_) => Ok(()),
        }
    }
}
