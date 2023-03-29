use serde::{Deserialize, Serialize};
use diesel::{Queryable, Insertable, AsChangeset};

use crate::schema::account_pubg;

#[derive(Serialize, Deserialize, Debug, Clone, Queryable, Insertable, AsChangeset)]
#[diesel(table_name = account_pubg)]
pub struct AccountPubg {
    #[serde(default)]
    pub pubg_id: String,
    // pub account_id: i32,
    pub name: String,
    // pub created_at: Option<chrono::NaiveDateTime>,
    // pub updated_at: Option<chrono::NaiveDateTime>,
}