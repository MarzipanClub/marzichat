use {
    crate::{actor, actor::ActorSender},
    actix_web::{
        http::{header::ContentType, StatusCode},
        web::Payload,
        HttpRequest, HttpResponse, HttpResponseBuilder, ResponseError,
    },
    actix_ws::MessageStream,
    anyhow::Result,
    common::{api::PING_PONG_PAYLOAD, types::account},
    futures::StreamExt,
    std::sync::{atomic::AtomicU32, Arc, OnceLock},
    tokio::sync::{Semaphore, TryAcquireError},
    uuid::Uuid,
};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// The ip address could not be parsed.
    #[error("Error parsing ip address: `{0}`")]
    IpAddress(#[from] crate::address::Error),

    /// The connection cannot be accepted because of rate limits.
    /// 429 Too Many Requests
    #[error("New connections are being rate limited.")]
    RateLimited,

    // The connection semaphore should never be closed. If it is, it's a bug in the server.
    #[error("The connection semaphore was erroneously closed.")]
    ConnectionSemaphoreClosed,

    #[error("Websocket handshake error: `{0}`")]
    WebsocketHandshake(#[from] actix_web::Error),
}

impl ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        let status = match self {
            Error::IpAddress(_) | Error::WebsocketHandshake(_) => StatusCode::BAD_REQUEST,
            Error::RateLimited => StatusCode::TOO_MANY_REQUESTS,
            Error::ConnectionSemaphoreClosed => {
                tracing::error!(
                    "500 Internal Server Error: the connection semaphore was erroneously closed."
                );
                StatusCode::INTERNAL_SERVER_ERROR
            }
        };

        HttpResponseBuilder::new(status)
            .content_type(ContentType::plaintext())
            .body(self.to_string())
    }
}

/// Get the connection count semaphore.
fn websocket_count_semaphore() -> Arc<Semaphore> {
    static SEMAPHORE: OnceLock<Arc<Semaphore>> = OnceLock::new();
    SEMAPHORE
        .get_or_init(|| {
            Arc::new(Semaphore::new(
                crate::config::get().max_websocket_connections,
            ))
        })
        .clone()
}

/// Stream the websocket messages to the actor.
/// Once the stream is done, the channel is closed.
fn stream_webapp_messages(client_id: u32, mut stream: MessageStream, actor_sender: ActorSender) {
    tokio::task::spawn_local(async move {
        while let Some(result) = stream.next().await {
            match result {
                Ok(actix_ws::Message::Pong(bytes)) if bytes == PING_PONG_PAYLOAD => {
                    if let Err(error) = actor_sender.pong().await {
                        tracing::error!(?error, "failed to send app message to actor");
                        break;
                    }
                }
                Ok(actix_ws::Message::Binary(bytes)) => match bincode::deserialize(&bytes) {
                    Ok(app_message) => {
                        if let Err(error) = actor_sender.send(app_message).await {
                            tracing::error!(?error, "failed to send app message to actor");
                            break;
                        }
                    }
                    Err(error) => {
                        tracing::error!(?error, "failed to deserialize app message");
                        break;
                    }
                },
                Ok(actix_ws::Message::Close(_)) => {
                    tracing::debug!(%client_id, "connection closed");
                    actor::remove(&client_id);
                    tracing::info!(count = actor::count(), "connected clients");
                    break;
                }
                message => {
                    tracing::error!(?message, "unknown app message");
                    break;
                }
            }
        }
    });
}

/// The handler for a websocket stream api.
pub async fn handler(request: HttpRequest, payload: Payload) -> Result<HttpResponse, Error> {
    let ip_address = crate::address::parse(&request)?;

    match websocket_count_semaphore().try_acquire_owned() {
        Ok(permit) => {
            // generate a new client id
            static CLIENT_ID_COUNTER: AtomicU32 = AtomicU32::new(0);

            let client_id = CLIENT_ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);

            tracing::info!(
                %client_id,
                %ip_address,
                "connection accepted"
            );

            // TODO: remove this nil uuid and use a real account id
            let account_id = account::Id(Uuid::nil());

            let (response, session, stream) = actix_ws::handle(&request, payload)?;

            stream_webapp_messages(
                client_id,
                stream,
                actor::spawn(client_id, session, Some(account_id), permit),
            );

            tracing::info!(count = actor::count(), "connected clients");

            Ok(response)
        }
        Err(TryAcquireError::NoPermits) => {
            tracing::warn!(
                %ip_address,
                "rate limiting"
            );
            Err(Error::RateLimited)
        }
        Err(TryAcquireError::Closed) => Err(Error::ConnectionSemaphoreClosed),
    }
}
