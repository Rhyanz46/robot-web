mod api;
mod validation;

use std::time::Duration;
use tokio::task;
use api::bot::player_data::PlayerData;
use actix_web::{get, post, web, App, HttpRequest, HttpServer, HttpResponse, middleware::Logger};
use std::thread;
use validation::{Buy, Res, CheckId};
use actix_cors::Cors;


#[get("/")]
async fn index(req: HttpRequest) -> &'static str {
    println!("REQ: {:?}", req);
    "Hello world!\r\n"
}

#[post("/buy")]
async fn buy(data: web::Json<Buy>) -> HttpResponse {
    let res = data.clone();
    task::spawn(async move {
        let player= PlayerData{
            pubg_id: &data.pubg_id, 
            hp_selected: &data.hp,
        };
        player.buy().await;
    });
    HttpResponse::Ok().json(Res{
        message:"Pembelian dalam proses".to_string(),
        data: res,
    })
}

#[post("/check-id")]
async fn check_id(data: web::Json<CheckId>) -> HttpResponse {
    let mut res = data.clone();
    let player= PlayerData{
        pubg_id: &data.pubg_id, 
        hp_selected: "",
    };
    let get_name_res = player.check_id().await;
    match get_name_res {
        Ok(get_name) => {
            res.player_name = Some(get_name);
            HttpResponse::Ok().json(Res{
                message:"Success".to_string(),
                data: res,
            })
        },
        Err(msg) => {
            HttpResponse::BadRequest().json(Res{
                message:msg,
                data: res,
            })
        }
    }
}

#[get("/test")]
async fn tess(_: HttpRequest) -> HttpResponse{
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_secs(4));
        }
    });
    HttpResponse::Ok().body("ok tess")
}

fn init_routes(config: &mut web::ServiceConfig) {
    config.service(index);
    config.service(buy);
    config.service(tess);
    config.service(check_id);
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header()
            )
        .configure(init_routes)
    })
    .bind(("0.0.0.0", 8888))?
    .run()
    .await
}