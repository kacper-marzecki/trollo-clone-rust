use actix_web::web;
use bcrypt::{DEFAULT_COST, hash};
use diesel::sql_query;
use futures_util::FutureExt;
use sqlx::{Connection, Executor, PgConnection, Pool, Transaction};
use sqlx::pool::PoolConnection;

use crate::api::user_api::UserRegisterRequest;
use crate::app_error::AppError;
use crate::repository::{user_repository, RepositoryImpl, Repository};
use crate::repository::user_repository::{CreateUserDto, UserRepository};
use crate::utils::respond_ok;
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

#[cfg(not(test))]
type Trans = Transaction<PoolConnection<PgConnection>>;
#[cfg(test)]
type Trans = tests::MockT;

pub async fn register_user(
    request: UserRegisterRequest,
    mut repository: RepositoryImpl
) -> Result<(), AppError>
{
    let exists = repository.exists_by_username_or_email(&request.username, &request.email).await?;
    if !exists {
        repository.create_user( CreateUserDto {
            password: request.password,
            email: request.email,
            username: request.username,
        }).await?;
        repository.commit().await;
        Ok(())
    } else {
        Err(AppError::ValidationError(vec!["Already exists".to_string()]))
    }
}


#[cfg(test)]
pub mod tests {
mock!{

}
    #[actix_rt::test]
    async fn doesnt_register_user_if_such_username_exists() {

        // mock!(Pool<PgConnection>)
        //     .
    }
}