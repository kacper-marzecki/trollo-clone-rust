use std::sync::Arc;
use std::borrow::BorrowMut;
use r2d2::PooledConnection;
use r2d2_postgres::PostgresConnectionManager;
use postgres::NoTls;

pub mod user_repository;

pub type ConnPool = r2d2::Pool<PostgresConnectionManager<NoTls>>;

// pub struct RepositoryImpl (pub Transaction<PoolConnection<PgConnection>>);
//
// #[async_trait]
// impl Repository for RepositoryImpl {
//     fn get(&mut self) -> &mut Transaction<PoolConnection<PgConnection>> {
//         self.0.borrow_mut()
//     }
//
//     async fn commit(self) -> () {
//         self.0.commit().await;
//     }
//
//     async fn rollback(self) -> () {
//         self.0.rollback().await;
//     }
// }
//
// #[async_trait]
// pub trait Repository {
//     fn get(&mut self) -> &mut Transaction<PoolConnection<PgConnection>>;
//     async fn commit(self) -> ();
//     async fn rollback(self) -> ();
// }

// pub struct TransactionSource {
//     pub transaction: Arc<Transaction<PoolConnection<PgConnection>>>
// }

// impl TransactionSource {
//     pub async fn commit(&self)-> Result<(), sqlx::Error> {
//         self.transaction.commit().await?;
//         Ok(())
//     }
//
//     pub async fn rollback(&self)-> Result<(), sqlx::Error> {
//         self.transaction.rollback().await?;
//         Ok(())
//     }
//
//     pub fn get(&mut self) -> &mut Transaction<PoolConnection<PgConnection>> {
//         &mut self.transaction
//     }
//
// }

// #[async_trait]
// pub trait Repository: UserRepository {
//     async fn begin(&self)-> Transaction<PoolConnection<PgConnection>>;
// }
//
// pub struct RepositoryImpl {
//     pub pool: Pool<PgConnection>
// }

// #[async_trait]
// impl Repository for RepositoryImpl {
//     async fn begin(&self) -> Transaction<PoolConnection<PgConnection>> {
//         self.begin().await
//     }
// }