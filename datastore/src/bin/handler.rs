use {
    anyhow::Result,
    datastore::{Datastore, Error},
    redb::{Database, ReadableTable, TableDefinition},
    serde::{de::DeserializeOwned, Deserialize, Serialize},
    tarpc::context,
    tokio::sync::mpsc::{Sender, UnboundedSender},
};
/// The various commands that for the service.
#[derive(Debug)]
pub enum Command {
    Get {
        table: String,
        key: String,
    },
    Set {
        table: String,
        key: String,
        value: String,
    },
    Delete {
        table: String,
        key: String,
    },
    List,
}

/// The context for a command.
/// The response sender is used to send the response back to the client.
#[derive(Debug)]
pub struct CommandContext {
    command: Command,
    response_sender: Sender<Option<String>>,
}

trait Store<'a, 'k, 'v>: Deserialize<'v> {
    const TABLE: TableDefinition<'a, &'static str, &'static [u8]>;
    type Response: Serialize;
}

impl CommandContext {
    pub fn handle(self, database: &Database) {
        let CommandContext {
            command,
            response_sender,
        } = self;

        match command {
            Command::Get { table, key } => {
                //
            }
            Command::Set { table, key, value } => {
                //
            }
            Command::Delete { table, key } => {
                //
            }
            Command::List => {
                //
            }
        }
    }
}

fn handle_get(
    _table_name: TableDefinition<&str, &[u8]>,
    table: &str,
    key: &str,
    database: &Database,
) -> Result<Option<String>, Error> {
    const TABLE: TableDefinition<&str, u64> = TableDefinition::new("my_data");

    let read_txn = database.begin_read()?;
    let table = read_txn.open_table(TableDefinition::<&str, &[u8]>::new(table))?;
    let maybe_value = table.get(key)?;
    match maybe_value {
        Some(access_guard) => {
            let value = access_guard.value();
            Ok(None)
        }
        None => Ok(None),
    }
}

/// A context for requests to the datastore service.
#[derive(Clone)]
pub struct Context {
    main_sender: UnboundedSender<CommandContext>,
}

impl Context {
    /// Creates a new handler.
    pub fn new(main_sender: UnboundedSender<CommandContext>) -> Self {
        Self { main_sender }
    }
}

#[tarpc::server]
impl Datastore for Context {
    async fn get(
        self,
        _: context::Context,
        table: String,
        key: String,
    ) -> Result<Option<String>, Error> {
        tracing::debug!("received request for key: {}", key);

        let (response_sender, mut response_receiver) = tokio::sync::mpsc::channel(1);

        self.main_sender
            .send(CommandContext {
                command: Command::Get { table, key },
                response_sender,
            })
            .map_err(|_| Error::Exiting)?;
        let response = response_receiver.recv().await.ok_or(Error::Exiting)?;

        Ok(response)
    }

    async fn set(
        self,
        _: context::Context,
        table: String,
        key: String,
        value: String,
    ) -> Result<Option<String>, Error> {
        tracing::debug!("received request to set key: {} to value: {}", key, value);
        Ok(Some("value".into()))
    }

    async fn delete(
        self,
        _: context::Context,
        table: String,
        key: String,
    ) -> Result<Option<String>, Error> {
        tracing::debug!("received request to delete key: {}", key);
        Ok(Some("value".into()))
    }

    async fn list(self, _: context::Context) -> Result<Vec<String>, Error> {
        tracing::debug!("received request to list tables");
        Ok(vec!["table".into()])
    }
}
