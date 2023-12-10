use sqlx::postgres::PgPool;
use tokio::task;
use futures::stream::{self, StreamExt};
use std::sync::{Arc, RwLock};

pub fn execute_queries_in_parallel(pool: &PgPool, queries: Arc<RwLock<Vec<String>>>) -> Result<(), sqlx::Error> {
    let rt = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(8)
        .build()
        .unwrap();

    rt.block_on(async {
        let tasks: Vec<_> = queries.read().unwrap().iter().map(|query| {
            let pool = pool.clone();
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