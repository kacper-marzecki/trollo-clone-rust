use actix_web::web;
use bcrypt::{DEFAULT_COST, hash};
use diesel::sql_query;
use futures_util::FutureExt;

use crate::api::user_api::UserRegisterRequest;
use crate::app_error::AppError;
use crate::repository::user_repository::{CreateUserDto, UserRepository};
use crate::utils::respond_ok;
use actix_web::web::Json;
use crate::validation::validate;

pub async fn register_user<T: UserRepository>(
    repository: &mut T,
    request: UserRegisterRequest,
) -> Result<(), AppError>
{
    validate(&request)?;
    let exists = repository.exists_by_username_or_email(&request.username, &request.email).await?;
    if !exists {
        repository.create_user(CreateUserDto {
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
    use std::cell::Cell;
    use std::rc::Rc;
    use std::sync::{Arc, Mutex};
    use std::sync::atomic::{AtomicUsize, Ordering};

    use deadpool_postgres::Transaction;
    use futures::future::ok;
    use futures_util::future::Map;
    use futures_util::future::Ready;
    use futures_util::FutureExt;
    use mocktopus::mocking::*;

    use crate::api::user_api::UserRegisterRequest;
    use crate::app_error::AppError;
    use crate::repository;
    use crate::repository::user_repository::{UserRepository, UserRepositoryImpl};
    use crate::service::user_service::register_user;
    use actix_web::web::Json;
    use serde::export::TryFrom;

    async fn asyncOk<T>(x: T) -> Result<T, AppError> {
        Ok(x)
    }

    fn counters() -> (Arc<Mutex<u8>>, Arc<Mutex<u8>>) {
        let mock = Arc::new(Mutex::new(0_u8));
        let clone = mock.clone();
        (mock, clone)
    }

    fn increment(counter: &mut Arc<Mutex<u8>>) {
        *counter.lock().unwrap() += 1;
    }

    fn contains_errors<T>(result: Result<T, AppError>, error_fragments: Vec<&str>) -> bool {
        if let Err(AppError::ValidationError(errors)) = result {
            error_fragments.iter().all(|expected| {
                errors.iter().any(|error| error.contains(expected))
            })
        } else {
            false
        }
    }

    #[actix_rt::test]
    async fn doesnt_register_user_if_such_username_exists() {
        // given
        let mut mock = UserRepositoryImpl { conn: None };
        let (mut calls, mut calls_clone) = counters();
        UserRepositoryImpl::exists_by_username_or_email
            .mock_safe(move |mock: &mut repository::user_repository::UserRepositoryImpl<'_, '_>, _, _| {
                increment(&mut calls_clone);
                MockResult::Return(Box::pin(asyncOk(true)))
            });
        // when
        let result = register_user(
            &mut mock, UserRegisterRequest { username: "AAAAAA".into(), email: "AAAAAA@asd.com".into(), password: "AAAAAA".into() },
        ).await;
        // then
        assert!(result.is_err());
        assert_eq!(*calls.lock().unwrap(), 1);
    }

    #[actix_rt::test]
    async fn registers_user_if_user_doesnt_exist() {
        // given
        let mut mock = UserRepositoryImpl { conn: None };
        let (mut calls, mut calls_clone) = counters();
        let (mut register_calls, mut register_calls_clone) = counters();
        UserRepositoryImpl::exists_by_username_or_email
            .mock_safe(move |mock: &mut repository::user_repository::UserRepositoryImpl<'_, '_>, _, _| {
                increment(&mut calls_clone);
                MockResult::Return(Box::pin(asyncOk(false)))
            });
        UserRepositoryImpl::create_user
            .mock_safe(move |mock: &mut repository::user_repository::UserRepositoryImpl<'_, '_>, _| {
                increment(&mut register_calls_clone);
                MockResult::Return(Box::pin(asyncOk(true)))
            });
        // when
        let result = register_user(
            &mut mock, UserRegisterRequest { username: "AAAAAA".into(), email: "AAAAAA@asd.com".into(), password: "AAAAAA".into() },
        ).await;
        // then
        assert!(result.is_ok());
        assert_eq!(*calls.lock().unwrap(), 1);
        assert_eq!(*register_calls.lock().unwrap(), 1);
    }

    #[actix_rt::test]
    async fn registration_validation_works() {
        // given
        let mut mock = UserRepositoryImpl { conn: None };
        // when
        let result = register_user(
            &mut mock, UserRegisterRequest { username: "".into(), email: "".into(), password: "".into() },
        ).await;
        // then
        assert!(contains_errors(result, vec!["username", "email", "password"]))
    }
}

