// #[macro_use]
// extern crate diesel;
// extern crate dotenv;

use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use dotenv::dotenv;
use std::env;
pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();
    let db_host = env::var("DB_HOST").expect("DB_HOST must be set");
    let db_port = env::var("DB_PORT").expect("DB_PORT must be set");
    let db_user = env::var("DB_USER").expect("DB_USER must be set");
    let db_password = env::var("DB_PASSWORD").expect("DB_PASSWORD must be set");
    let db_name = env::var("DB_NAME").expect("DB_NAME must be set");
    let db_url = format!("mysql://{}:{}@{}:{}/{}", db_user, db_password, db_host, db_port, db_name); 
      MysqlConnection::establish(&db_url)
        .expect(&format!("Error connecting to {}", db_url))
}