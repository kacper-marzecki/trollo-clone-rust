use actix_web::HttpResponse;
use actix_web::body::Body;
use crate::app_error::AppError;

pub fn is_blank(it: &String) -> bool {
    return it.is_empty()
        || is_whitespace(it)
}

pub fn is_whitespace(it: &String) -> bool {
    it.trim().is_empty()
}


pub fn respond_ok() -> Result<HttpResponse, AppError> {
    Ok(HttpResponse::Ok().body(Body::Empty))
}