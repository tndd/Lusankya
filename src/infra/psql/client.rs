use std::sync::{Arc, Mutex};
use tokio_postgres::{Client, NoTls, Error};

pub struct PsqlClient {
    client: Arc<Mutex<Client>>,
}

impl PsqlClient {
    pub async fn new_async(host: String, user: String, password: String, db_name: String) -> Result<Self, Error> {
        let conn_string = format!("host={} user={} password={} dbname={}", host, user, password, db_name);
        let (client, connection) = tokio_postgres::connect(&conn_string, NoTls).await?;

        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("connection error: {}", e);
            }
        });

        Ok(Self {
            client: Arc::new(Mutex::new(client)),
        })
    }
}