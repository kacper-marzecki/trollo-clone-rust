use crate::model::{Board, Card, CardTaskItem, Lane, User};
use actix_web::web::{get, ServiceConfig};
use actix_web::{error, middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer, Scope};
pub use log::{error, info, trace, warn};
use serde::{Deserialize, Serialize};

pub fn lane_routes() -> Scope {
    web::scope("/lane").route("/{laneId}", web::get().to(get_lane_by_id))
}

async fn get_lane_by_id(r: HttpRequest) -> HttpResponse {
    info!("get_lane_by_id");
    let id = &r.match_info()["laneId"];
    HttpResponse::Ok().json(Lane {
        id: id.to_string(),
        board_id: "String".to_string(),
        name: "String".to_string(),
        position_in_board: 3,
    })
}
