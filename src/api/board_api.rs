use crate::model::{Board, Card, CardTaskItem, Lane, User};
use actix_web::web::{get, ServiceConfig};
use actix_web::{error, middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer, Scope};
pub use log::{error, info, trace, warn};
use serde::{Deserialize, Serialize};

pub fn board_routes() -> Scope {
    web::scope("/board")
        .route("", web::get().to(get_boards))
        .route("/{boardId}", web::get().to(get_board_by_id))
        .route("/{boardId}/lanes", web::get().to(get_board_lanes))
}

async fn get_boards() -> HttpResponse {
    info!("get_boards");
    HttpResponse::Ok().json(vec![
        Board {
            id: "boardId1".to_string(),
            name: "boardName".to_string(),
        },
        Board {
            id: "boardId2".to_string(),
            name: "boardName".to_string(),
        },
    ])
}

async fn get_board_by_id(r: HttpRequest) -> HttpResponse {
    info!("get_board_by_id");
    let id = &r.match_info()["boardId"];
    HttpResponse::Ok().json(Board {
        id: id.to_string(),
        name: "boardName".to_string(),
    })
}

async fn get_board_lanes(r: HttpRequest) -> HttpResponse {
    info!("get_board_lanes");
    HttpResponse::Ok().json(vec![
        Lane {
            id: "1".to_string(),
            board_id: "String".to_string(),
            name: "String".to_string(),
            position_in_board: 3,
        },
        Lane {
            id: "2".to_string(),
            board_id: "String".to_string(),
            name: "String".to_string(),
            position_in_board: 3,
        },
    ])
}
