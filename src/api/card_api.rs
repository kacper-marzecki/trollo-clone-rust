use crate::model::{Board, Card, CardTaskItem, Lane, User};
use actix_web::web::{get, ServiceConfig};
use actix_web::{error, middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer, Scope};
pub use log::{error, info, trace, warn};
use serde::{Deserialize, Serialize};

pub fn card_routes() -> Scope {
    web::scope("/card")
        .route("/{cardId}", web::get().to(get_card_by_id))
        .route("/{cardId}/tasks", web::get().to(get_card_tasks))
}

async fn get_card_by_id(r: HttpRequest) -> HttpResponse {
    info!("get_card_by_id");
    let id = &r.match_info()["cardId"];

    HttpResponse::Ok().json(Card {
        id: id.to_string(),
        name: "card name".to_string(),
        description: "description".to_string(),
        position_in_lane: 3,
        files: vec!["asd".to_string(), "file2".to_string(), "file3".to_string()],
    })
}

async fn get_card_tasks(r: HttpRequest) -> HttpResponse {
    info!("get_card_tasks");
    let id = &r.match_info()["cardId"];

    HttpResponse::Ok().json(vec![
        CardTaskItem {
            id: "1".to_string(),
            card_id: id.to_string(),
            text: "some task".to_string(),
            is_complete: false,
        },
        CardTaskItem {
            id: "2".to_string(),
            card_id: id.to_string(),
            text: "some other task".to_string(),
            is_complete: true,
        },
    ])
}
