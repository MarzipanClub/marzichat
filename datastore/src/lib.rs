#[tarpc::service]
pub trait Datastore {
    /// Gets the value of key.
    async fn get(key: String) -> String;

    /// Sets the value of key to value and returns the previous value if it was
    /// set.
    async fn set(key: String, value: String) -> Option<String>;
}
