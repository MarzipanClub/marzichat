use {
    crate::actor::Context,
    anyhow::Result,
    common::api::{AppMessage, BackendMessage},
};

/// Processes a message from the app.
///
/// Errors are not propagated to the app.
/// Consider them nonrecoverable and not part of the api.
/// Any error should cause the actor to terminate since the backend cannot
/// fullfil its api contract.
pub async fn process(message: AppMessage, context: &mut Context) -> Result<()> {
    match message {
        // AppMessage::Ping => {
        //     context.send(&BackendMessage::Pong).await?;
        // }
        // AppMessage::Heartbeat => {
        //     context.heartbeat()?;
        // }
        AppMessage::GenerateUsername => {
            context
                .send(&BackendMessage::GeneratedUsername("foobar".into()))
                .await?;
        }
        AppMessage::CheckUsernameAvailability(username) => {
            // let is_available = username::is_available(&username).await?;
            let is_available = true;
            context
                .send(&BackendMessage::UsernameAvailability((
                    username,
                    is_available,
                )))
                .await?;
        }
    }
    Ok(())
}
