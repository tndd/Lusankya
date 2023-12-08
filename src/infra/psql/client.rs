use std::sync::{Arc, RwLock};
use tokio_postgres::{Client, Error, NoTls};

pub struct PsqlClient {
    client: Arc<RwLock<Client>>,
}

impl PsqlClient {
    pub fn new(client: Client) -> Self {
        PsqlClient {
            client: Arc::new(RwLock::new(client)),
        }
    }
}
