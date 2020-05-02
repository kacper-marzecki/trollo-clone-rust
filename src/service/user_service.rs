use actix_web::web;
use bcrypt::{DEFAULT_COST, hash};
use diesel::sql_query;
use futures_util::FutureExt;

use crate::api::user_api::UserRegisterRequest;
use crate::app_error::AppError;
use crate::utils::respond_ok;
use crate::repository::user_repository::{CreateUserDto, UserRepository};
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



pub fn register_user(
    repository: &mut UserRepository,
    request: UserRegisterRequest
) -> Result<(), AppError>
{
    let exists = repository.exists_by_username_or_email(&request.username, &request.email)?;
    if !exists {
        repository.create_user( CreateUserDto {
            password: request.password,
            email: request.email,
            username: request.username,
        })?;
        Ok(())
    } else {
        Err(AppError::ValidationError(vec!["Already exists".to_string()]))
    }
}


#[cfg(test)]
pub mod tests {
mock!{
    pub MockRepo{}

}
    #[actix_rt::test]
    async fn doesnt_register_user_if_such_username_exists() {
        // let asd = MockUserRepository::new();
        // mock!(Pool<PgConnection>)
        //     .
    }
}