// @generated automatically by Diesel CLI.

diesel::table! {
    account_pubg (pubg_id) {
        pubg_id -> Varchar,
        name -> Varchar,
    }
}

diesel::table! {
    buy_history (id) {
        id -> Integer,
        pubg_id -> Varchar,
        status -> Varchar,
        message -> Nullable<Text>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    account_pubg,
    buy_history,
);
