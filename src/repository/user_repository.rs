use crate::app_error::AppError;
use async_trait::async_trait;
use futures_util::future::BoxFuture;
use std::convert::TryInto;
use actix_web::dev::Url;
use std::borrow::BorrowMut;
use std::pin::Pin;
use std::future::Future;
use r2d2::PooledConnection;
use chrono::{DateTime, Local, TimeZone};
use postgres::Transaction;
use std::sync::Arc;
use std::rc::Rc;

pub struct UserRepository<'a, 'b>(pub &'a mut Transaction<'b>);

impl UserRepository<'_, '_>  {
    pub fn create_user(&mut self, dto: CreateUserDto) -> Result<bool, AppError> {
        let created = self.0.execute("
        insert into users (username, email, password, avatar_id, created_at)
                                values ($1, $2, $3, NULL, $4);
          ", &[&dto.username, &dto.email, &dto.password, &chrono::Utc::now().timestamp()], )
            .map(|it| {
                match it {
                    1 => true,
                    _ => false
                }
            })?;
        Ok(created)
    }

    pub fn exists_by_username_or_email(&mut self, username: &String, email: &String) -> Result<bool, AppError> {
        let mut a = self.0.query("
        select exists(select 1 from users where username = $1 or email = $2)
    ", &[&username, &email])
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