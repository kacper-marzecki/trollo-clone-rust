use std::borrow::BorrowMut;
use std::convert::TryInto;
use std::future::Future;
use std::pin::Pin;
use std::rc::Rc;
use std::sync::Arc;

use actix_web::dev::Url;
use chrono::{DateTime, Local, TimeZone};
use futures_util::future::BoxFuture;
use tokio_postgres::Transaction;

use async_trait::async_trait;

use crate::app_error::AppError;

pub struct UserRepository<'a, 'b>(pub &'a mut Transaction<'b>);

impl UserRepository<'_, '_>  {
    pub async  fn create_user(&mut self, dto: CreateUserDto) -> Result<bool, AppError> {
        let created = self.0.execute("
        insert into users (username, email, password, avatar_id, created_at)
                                values ($1, $2, $3, NULL, $4);
          ", &[&dto.username, &dto.email, &dto.password, &chrono::Utc::now().timestamp()], )
            .await
            .map(|it| {
                match it {
                    1 => true,
                    _ => false
                }
            })?;
        Ok(created)
    }

    pub async  fn exists_by_username_or_email(&mut self, username: &String, email: &String) -> Result<bool, AppError> {
        let mut a = self.0.query("
        select exists(select 1 from users where username = $1 or email = $2)
    ", &[&username, &email])
            .await
            .map(|rows|{
                match rows.first() {
                    Some(x) => true,
                    None => false
                }
            })?;
        Ok(a)
    }
}

// INPUT
pub struct CreateUserDto {
    pub username: String,
    pub email: String,
    pub password: String,
}

// OUTPUT
struct Exists {
    pub exists: bool
}