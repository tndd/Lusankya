use tokio_postgres::{NoTls, Error};
use dotenv::dotenv;

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv().ok();
    // Connect to the database
    let db_host = std::env::var("DB_HOST").expect("DB_HOST must be set");
    let db_user = std::env::var("DB_USER").expect("DB_USER must be set");
    let db_password = std::env::var("DB_PASSWORD").expect("DB_PASSWORD must be set");
    let db_name = std::env::var("DB_NAME").expect("DB_NAME must be set");
    let (client, connection) = tokio_postgres::connect(&format!("host={} user={} password={} dbname={}", db_host, db_user, db_password, db_name), NoTls).await?;

    // The connection object performs the actual communication with the database,
    // so spawn it off to run on its own.
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    // Now we can execute a query with the client
    let rows = client.query("SELECT table_name FROM information_schema.tables WHERE table_schema='public'", &[]).await?;
    for row in &rows {
        let table_name: &str = row.get(0);
        println!("{}", table_name);
    }

    Ok(())
}