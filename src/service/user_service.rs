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
// #[cfg(not(test))]
type UserRepo<'a, 'b> = UserRepository<'a, 'b>;
// #[cfg(test)]
// type UserRepo<'a> = crate::repository::user_repository::MockUserRepository<'a>;

pub async fn register_user(
    repository: &mut UserRepo<'_, '_>,
    request: UserRegisterRequest
) -> Result<(), AppError>
{
    let exists = repository.exists_by_username_or_email(&request.username, &request.email).await?;
    if !exists {
        repository.create_user( CreateUserDto {
            password: request.password,
            email: request.email,
            username: request.username,
        }).await?;
        Ok(())
    } else {
        Err(AppError::ValidationError(vec!["Already exists".to_string()]))
    }
}


#[cfg(test)]
pub mod tests {
    use mocktopus::mocking::*;
    use crate::service::user_service::{UserRepo, register_user};
    use crate::repository::user_repository::UserRepository;
    use deadpool_postgres::Transaction;
    use crate::api::user_api::UserRegisterRequest;
    use futures::future::ok;
    use futures_util::future::Ready;
    use futures_util::FutureExt;
    use futures_util::future::Map;
    use crate::app_error::AppError;

    // use futures_util::future::ok;
async fn asyncOk()->Result<bool, AppError> {
        Ok(true)
    }
    #[actix_rt::test]
    async fn doesnt_register_user_if_such_username_exists() {
        let mut mock = UserRepository(None);
        // UserRepository::exists_by_username_or_email.mock_safe(|_, _, _| MockResult::Return(ok(Ok(true)).map(|x|x).boxed()));
        UserRepository::exists_by_username_or_email.mock_safe(|_, _, _| MockResult::Return(asyncOk()));
        let result = register_user(&mut mock, UserRegisterRequest{username: "".into(), email: "".into(), password:"".into()}).await;
        assert!(result.is_err());
        // let mut mock = MockUserRepository::default();
        // mock.expect_exists_by_username_or_email().returning(|_, _| Ok(true));
        // assert_eq!(mock.exists_by_username_or_email("asd".to_string(), "b".to_string()), Ok(true));
        println!("asd");
        // let asd = MockUserRepository::new();
        // mock!(Pool<PgConnection>)
        //     .
    }
}