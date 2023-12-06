use tokio_postgres::{Client, NoTls, Error};

pub struct PsqlClient {
    host: String,
    user: String,
    password: String,
    db_name: String
}

impl PsqlClient {
    pub fn new(host: String, user: String, password: String, db_name: String) -> Self {
        Self {
            host,
            user,
            password,
            db_name
        }
    }

    pub async fn connect(&self) -> Result<Client, Error> {
        let conn_string = format!(
            "host={} user={} password={} dbname={}",
            self.host,
            self.user,
            self.password,
            self.db_name
        );
        let (client, connection) = tokio_postgres::connect(&conn_string, NoTls).await?;

        // The connection object performs the actual communication with the database,
        // so spawn it off to run on its own.
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("connection error: {}", e);
            }
        });

        Ok(client)
    }
}