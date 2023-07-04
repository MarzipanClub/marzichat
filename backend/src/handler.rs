use {
    crate::{actor::Context, username},
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
        AppMessage::SuggestUsername => {
            context
                .send(&BackendMessage::SuggestedUsername(username::suggested()?))
                .await?;
        }
        AppMessage::CheckUsernameAvailability(username) => {
            let is_available = username::is_available(&username).await?;
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
