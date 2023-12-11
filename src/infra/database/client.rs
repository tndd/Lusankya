use sqlx::postgres::PgPool;
use tokio::task;
use futures::stream::{self, StreamExt};
use std::sync::Arc;
use std::sync::RwLock;

pub struct DbClient {
    pool: PgPool,
}

impl DbClient {
    pub fn new(database_url: &str) -> Result<Self, sqlx::Error> {
        let rt = tokio::runtime::Runtime::new().unwrap();
        let pool = rt.block_on(PgPool::connect(database_url))?;
        Ok(Self { pool })
    }

    pub fn execute_queries_in_parallel(&self, queries: Arc<RwLock<Vec<String>>>) -> Result<(), sqlx::Error> {
        let rt = tokio::runtime::Builder::new_multi_thread()
            .worker_threads(8)
            .build()
            .unwrap();

        rt.block_on(async {
            let tasks: Vec<_> = queries.read().unwrap().iter().map(|query| {
                let pool = self.pool.clone();
                let query = query.clone();
                task::spawn(async move {
                    let rows_affected = sqlx::query(&query).execute(&pool).await?.rows_affected();
                    Ok::<_, sqlx::Error>(rows_affected)
                })
            }).collect();

            let results = stream::iter(tasks).buffer_unordered(8).collect::<Vec<_>>().await;

            for result in results {
                match result {
                    Ok(Ok(_)) => (),
                    Ok(Err(e)) => return Err(e),
                    Err(e) => return Err(sqlx::Error::Protocol(e.to_string().into()))
                }
            }

            Ok(())
        })
    }
}


    pub async fn execute_queries_in_parallel_another<F, R>(
        &self,
        queries: &[String],
        execute_and_process: F,
    ) -> Result<Vec<R>, sqlx::Error>
    where
        F: Fn(&str) -> R + Send + Sync + 'static,
        R: Future<Output = Result<(), sqlx::Error>> + Send + 'static,
    {
        let tasks: Vec<_> = queries.iter().map(|query| {
            let pool = self.pool.clone();
            let query = query.clone();
            tokio::spawn(async move {
                let result = execute_and_process(&query).await?;
                Ok::<_, sqlx::Error>(result)
            })
        }).collect();

        let results = futures::future::join_all(tasks).await;
        // Handle results...
    }

let queries = vec![
    "SELECT * FROM table1",
    "SELECT * FROM table2",
    // ...
];

let results = db_client.execute_queries_in_parallel(&queries, |query| async move {
    let rows = sqlx::query(query).fetch_all(&db_client.pool).await?;
    Ok(rows)
}).await?;