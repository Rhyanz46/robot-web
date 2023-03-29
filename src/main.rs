mod api;
mod validation;
mod db;
mod schema;
mod models;

use std::time::Duration;
use tokio::task;
use api::bot::player_data::{{PlayerData, get_port, put_back_port}};
// use actix::{Actor, Addr, System, SystemExit};
use actix_web::{get, post, web, App, HttpRequest, HttpServer, HttpResponse, middleware::Logger};
use actix_web::web::Data;
use std::thread;
use validation::{Buy, Res, CheckId};
use actix_cors::Cors;

use db::{Database};


#[get("/")]
async fn index(req: HttpRequest) -> String {
    // match get_port() {
    //     Some(res) => {
    //         // put_back_port(res.clone());
    //         res
    //     },
    //     None => "gk ada".to_string()
    // }
    "aaa".to_string()

}

#[post("/buy")]
async fn buy(data: web::Json<Buy>) -> HttpResponse {
    let res = data.clone();
    task::spawn(async move {
        let player= PlayerData{
            pubg_id: &data.pubg_id, 
            hp_selected: &data.hp,
            uc_selected: &data.uc,
        };
        player.buy().await;
    });
    HttpResponse::Ok().json(Res{
        message:"Pembelian dalam proses".to_string(),
        data: res,
    })
}

#[post("/check-id")]
async fn check_id(data: web::Json<CheckId>, db: Data<Database>) -> HttpResponse {
    let mut res = data.clone();
    let player= PlayerData{
        pubg_id: &data.pubg_id,
        hp_selected: "",
        uc_selected: "",
    };

    let get_name_res = player.check_id(db).await;
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
    HttpResponse::Ok().body("ok tes public")
}

fn init_routes(config: &mut web::ServiceConfig) {
    config.service(index);
    config.service(buy);
    config.service(tess);
    config.service(check_id);
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let db = Database::new();
    let db_pool = Data::new(db);

    println!("starting: v1");

    HttpServer::new(move || {
        App::new()
            .app_data(db_pool.clone())
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