pub use log::{error, info, trace, warn};
mod model;
use crate::model::{Board, Card, CardTaskItem, Lane, User};
use actix_web::web::get;
use actix_web::{error, middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct MyObj {
    name: String,
    number: i32,
}

/// This handler uses json extractor
async fn index(item: web::Json<MyObj>) -> HttpResponse {
    info!("model: {:?}", &item);
    HttpResponse::Ok().json(item.0)
}
async fn get_index() -> HttpResponse {
    let a = MyObj {
        name: "jej".to_string(),
        number: 1,
    };
    info!("model: {:?}", a);
    HttpResponse::Ok().json(a)
}

async fn get_current_user() -> HttpResponse {
    info!("get_current_user");
    HttpResponse::Ok().json(User {
        id: "userId".to_string(),
        name: "username".to_string(),
        email: "email@email.com".to_string(),
        avatar_id: "avatarId".to_string(),
        roles: vec!["USER".to_string()],
    })
}

async fn get_user_by_id(r: HttpRequest) -> HttpResponse {
    info!(
        "get_user_by_id {}",
        &r.match_info()["userId"].parse::<u8>().unwrap()
    );
    let id = &r.match_info()["userId"];
    HttpResponse::Ok().json(User {
        id: id.to_string(),
        name: "username".to_string(),
        email: "email@email.com".to_string(),
        avatar_id: "avatarId".to_string(),
        roles: vec!["USER".to_string()],
    })
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

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "info,actix_web=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(
                web::scope("/user")
                    .route("", web::get().to(get_current_user))
                    .route("/{userId}", web::get().to(get_user_by_id)),
            )
            .service(
                web::scope("/board")
                    .route("", web::get().to(get_boards))
                    .route("/{boardId}", web::get().to(get_board_by_id))
                    .route("/{boardId}/lanes", web::get().to(get_board_lanes)),
            )
            .service(web::scope("/lane").route("/{laneId}", web::get().to(get_lane_by_id)))
            .service(
                web::scope("/card")
                    .route("/{cardId}", web::get().to(get_card_by_id))
                    .route("/{cardId}/tasks", web::get().to(get_card_tasks)),
            )
        // .service(web::resource("/")
        //     .route(web::post().to(index))
        //     .route(web::get().to(get_index)))
    })
    .bind(format!(
        "0.0.0.0:{}",
        std::env::var("PORT").unwrap_or("8080".to_string()).as_str()
    ))?
    .run()
    .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::dev::Service;
    use actix_web::{http, test, web, App};

    #[actix_rt::test]
    async fn test_index() -> Result<(), Error> {
        let mut app =
            test::init_service(App::new().service(web::resource("/").route(web::post().to(index))))
                .await;

        let req = test::TestRequest::post()
            .uri("/")
            .set_json(&MyObj {
                name: "my-name".to_owned(),
                number: 43,
            })
            .to_request();
        let resp = app.call(req).await.unwrap();

        assert_eq!(resp.status(), http::StatusCode::OK);

        let response_body = match resp.response().body().as_ref() {
            Some(actix_web::body::Body::Bytes(bytes)) => bytes,
            _ => panic!("Response error"),
        };

        assert_eq!(response_body, r##"{"name":"my-name","number":43}"##);

        Ok(())
    }
}
