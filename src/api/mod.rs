use crate::api::board_api::board_routes;
use crate::api::card_api::card_routes;
use crate::api::lane_api::lane_routes;
use crate::api::user_api::user_routes;
use actix_web::web::ServiceConfig;

pub mod board_api;
pub mod card_api;
pub mod lane_api;
pub mod user_api;

pub fn routes(config: &mut ServiceConfig) {
    config
        .service(card_routes())
        .service(lane_routes())
        .service(user_routes())
        .service(board_routes());
}
