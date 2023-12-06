use tokio_postgres::{NoTls, Error};

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Connect to the database
    let (client, connection) = tokio_postgres::connect("host=localhost user=postgres password=secret dbname=test", NoTls).await?;

    // The connection object performs the actual communication with the database,
    // so spawn it off to run on its own.
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    // Now we can execute a query with the client
    let rows = client.query("SELECT * FROM my_table", &[]).await?;

    for row in rows {
        let value: i32 = row.get(0);
        println!("value: {}", value);
    }

    Ok(())
}