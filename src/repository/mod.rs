use sqlx::{PgConnection, Pool, Transaction};
use sqlx::pool::PoolConnection;
use crate::repository::user_repository::UserRepository;

pub mod user_repository;

#[async_trait]
pub trait Repository: UserRepository {
    async fn begin(&self)-> Transaction<PoolConnection<PgConnection>>;
}

pub struct RepositoryImpl {
    pub pool: Pool<PgConnection>
}

#[async_trait]
impl Repository for RepositoryImpl {
    async fn begin(&self) -> Transaction<PoolConnection<PgConnection>> {
        self.begin().await
    }
}