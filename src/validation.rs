use serde::{Deserialize, Serialize};
// use validator::Validate;

#[derive(Deserialize, Serialize, Clone)]
pub struct Buy {
    pub hp: String,
    pub pubg_id: String,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct CheckId {
    pub pubg_id: String,
    #[serde(default)]
    pub player_name: Option<String>,
}

#[derive(Serialize)]
pub struct Res<T> {
    pub message: String,
    pub data: T
}
