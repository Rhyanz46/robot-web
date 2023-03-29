// #[macro_use]
// extern crate diesel;
// extern crate dotenv;

use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use diesel::r2d2::{self, ConnectionManager};
use dotenv::dotenv;
use std::env;

use super::models::pubg_account::AccountPubg;
use crate::schema::account_pubg;

// struct DbExecutor(pub r2d2::Pool<ConnectionManager<MysqlConnection>>);
pub type DBPool = r2d2::Pool<ConnectionManager<MysqlConnection>>;

#[derive(Debug)]
pub struct Database {
    pool: DBPool,
}


impl Database {
    pub fn new() -> Self {
        dotenv().ok();

        let db_host = env::var("DB_HOST").expect("DB_HOST must be set");
        let db_port = env::var("DB_PORT").expect("DB_PORT must be set");
        let db_user = env::var("DB_USER").expect("DB_USER must be set");
        let db_password = env::var("DB_PASSWORD").expect("DB_PASSWORD must be set");
        let db_name = env::var("DB_NAME").expect("DB_NAME must be set");
        // let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let db_url = format!("mysql://{}:{}@{}:{}/{}", db_user, db_password, db_host, db_port, db_name);

        // let manager = ConnectionManager::<MysqlConnection>::new(db_url);
        // let pool = r2d2::builder().max_size(5).build(manager).expect("Failed to create pool");

        let manager = ConnectionManager::<MysqlConnection>::new(db_url);
        let pool: DBPool = r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create pool.");
        Database { pool }
    }

    pub fn set_pubg_account(&self, name: String, pubg_id: String) {
        let new_account = AccountPubg{
            name: name.to_string(),
            // created_at: None,
            // updated_at: None,
            pubg_id: pubg_id.to_string()
        };

        diesel::insert_into(account_pubg::table).
            values(&new_account).
            execute(&mut self.pool.get().unwrap()).
            expect("Error Bro");
    }

    pub fn get_pubg_account(&self, pubg_idx: String) -> Option<AccountPubg> {
        use crate::schema::account_pubg::dsl::*;
        let result_query = account_pubg.filter(pubg_id.eq(pubg_idx.clone()))
            .limit(1)
            .first::<AccountPubg>(&mut self.pool.get().unwrap())
            .optional()
            .expect("Error loading posts");

        match result_query {
            Some(result) => Some(result),
            None => None
        }
    }

    // pub fn get_db(&self) -> &DBPool {
    //     return &self.pool
    // }
}