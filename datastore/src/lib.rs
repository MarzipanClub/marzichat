use {
    anyhow::Result, bytesize::ByteSize, std::net::SocketAddr, tarpc::tokio_serde::formats::Bincode,
};

mod error;

pub use error::Error;
use serde::{de::DeserializeOwned, Deserialize};

trait Foo: DeserializeOwned {
    fn baz(&self);
}

#[tarpc::service]
pub trait Datastore {
    /// Gets the value of key.
    async fn get(table: String, key: String, foo: F) -> Result<Option<String>, Error>;

    /// Sets the value of key to value and returns the previous value if it was
    /// set.
    async fn set(table: String, key: String, value: String) -> Result<Option<String>, Error>;

    /// Deletes the value of key and returns the previous value if it was set.
    async fn delete(table: String, key: String) -> Result<Option<String>, Error>;

    /// Lists all the tables in the datastore.
    async fn list() -> Result<Vec<String>, Error>;
}

/// The max frame length to use.
const MAX_FRAME_LENGTH: ByteSize = ByteSize::mb(64);

/// Creates a client to the datastore.
pub async fn client(socket_address: SocketAddr) -> Result<DatastoreClient> {
    let mut transport = tarpc::serde_transport::tcp::connect(socket_address, Bincode::default);
    transport
        .config_mut()
        .max_frame_length(MAX_FRAME_LENGTH.as_u64() as usize);

    let client = DatastoreClient::new(tarpc::client::Config::default(), transport.await?).spawn();
    Ok(client)
}
