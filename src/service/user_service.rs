use actix_web::web;
use bcrypt::{DEFAULT_COST, hash};
use diesel::sql_query;
use futures_util::FutureExt;
use sqlx::{Connection, Executor, PgConnection, Pool, Transaction};
use sqlx::pool::PoolConnection;

use crate::api::user_api::UserRegisterRequest;
use crate::app_error::AppError;
use crate::repository::user_repository;
use crate::repository::user_repository::{CreateUserDto, UserRepository};
use crate::utils::respond_ok;
use crate::repository::Repository;
// async fn in_transaction<T, F>(
//     conn: PoolConnection<PgConnection>,
//     function: F,
// ) -> Result<T, AppError>
// where F:  FnOnce(&mut Transaction<PoolConnection<PgConnection>>) -> Result<T, AppError> {
//     let transaction = conn.begin().await?;
//     let result = function(transaction).await?;
//     transaction.commit().await?;
//     Ok(result)
// }

pub async fn register_user(
    request: UserRegisterRequest,
    repository: &Box<impl Repository>,
) -> Result<(), AppError>
{
    let mut trans = repository.begin().await;
    let exists = repository.exists_by_username_or_email(&mut trans, &request.username, &request.email).await?;
    if !exists {
        repository.create_user(&mut trans, CreateUserDto {
            password: request.password,
            email: request.email,
            username: request.username,
        }).await?;
        trans.commit().await?;
        Ok(())
    } else {
        Err(AppError::ValidationError(vec!["Already exists".to_string()]))
    }
}


#[cfg(test)]
pub mod tests {
    use sqlx::pool::PoolConnection;
    use crate::app_error::AppError;
    use sqlx::{PgConnection, Transaction, FromRow,QueryAs};
    use sqlx::prelude::PgQueryAs;
    use async_trait::async_trait;
    use crate::repository::{Repository, RepositoryImpl};
    use crate::repository::user_repository::{CreateUserDto, UserRepository};
    type Conn = PoolConnection<PgConnection>;
    type Trans = Transaction<Conn>;

    mock!{
        pub Trans {}


    }

    mock! {
    pub C {
    fn create_user(
        &self,
        conn: &mut Trans,
        dto: CreateUserDto
    ) -> Result<u64, AppError>;
    fn exists_by_username_or_email(
        &self,
        conn: &mut Trans,
        username: &String,
        email: &String
    ) -> Result<bool, AppError>;
        fn begin(&self)-> Transaction<PoolConnection<PgConnection>>;
    }
}

    #[actix_rt::test]
    async fn doesnt_register_user_if_such_username_exists() {
        impl Repository for MockC {
            async fn begin(&self) -> Transaction<PoolConnection<PgConnection>> {

            }
        }
        let mut asd = MockC::new();

        // mock!(Pool<PgConnection>)
        //     .
    }
}