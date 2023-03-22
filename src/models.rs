use diesel::Queryable;

#[derive(Queryable)]
pub struct PubgUser {
    pub pubg_id: Option<String>,
    pub pubg_name: Option<String>
}