use {
    anyhow::Result, bytesize::ByteSize, std::net::SocketAddr, tarpc::tokio_serde::formats::Bincode,
};

#[tarpc::service]
pub trait Datastore {
    /// Gets the value of key.
    async fn get(key: String) -> Option<String>;

    /// Sets the value of key to value and returns the previous value if it was
    /// set.
    async fn set(key: String, value: String) -> Option<String>;
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
