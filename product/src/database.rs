use std::{error::Error, sync::Arc};

use dashmap::DashMap;

use sqlx::postgres::{PgPool, PgPoolOptions, PgListener};
use uuid::Uuid;

use crate::models::{Product, NewProduct};

pub enum PersistenceError {
    UniqueConstraintViolation,
    NotFound,
    DatabaseError(Box<dyn Error + Send + Sync>),
}

impl From<sqlx::Error> for PersistenceError {
    fn from(error: sqlx::Error) -> Self {
        match error {
            sqlx::Error::Database(err) if err.is_unique_violation() => {
                PersistenceError::UniqueConstraintViolation
            } 
            _ => PersistenceError::DatabaseError(Box::new(error))
        }
    }
}

type PersistenceResult<T> = Result<T, PersistenceError>;

pub struct PostgresRepository {
    pool: PgPool,
    product_cache: Arc<DashMap<Uuid, Product>>,
}

impl PostgresRepository {
    pub async fn connect(url: &str, pool_size: u32) -> PersistenceResult<Self> {
        let pool = PgPoolOptions::new()
            .max_connections(pool_size)
            .connect(url)
            .await?;

        let product_cache = Arc::new(DashMap::with_capacity(30_000));
        
        tokio::spawn({
            let pool = pool.clone();
            let product_cache = product_cache.clone();

            async move {
                if let Ok(mut listener) = PgListener::connect_with(&pool).await {
                    listener.listen("new_product").await.ok();
                    while let Ok(message) = listener.recv().await {
                        if let Ok(product) = serde_json::from_str::<Product>(message.payload()) {
                            product_cache.insert(product.id, product);
                        }
                    }
                }
            }
        });

        Ok(PostgresRepository {
            pool,
            product_cache,
        })
    }

}


pub struct ProductDAO {
    connection: Arc<PostgresRepository>,
}

impl ProductDAO {
    pub fn new(connection: Arc<PostgresRepository>) -> Self {
        Self {
            connection,
        }
    }

    pub async fn count(&self) -> PersistenceResult<u64> {
        todo!()
    }

    pub async fn list(&self, query: &str) -> PersistenceResult<Vec<Product>> {
        todo!()
    }
    
    pub async fn get(&self, id: Uuid) -> PersistenceResult<Option<Product>> {
        todo!()
    }

    pub async fn create(&self, product: NewProduct) -> PersistenceResult<Product> {
        todo!()
    }
}

