use std::env;
use sqlx::postgres::PgPoolOptions;
use theeka_api::api;

const SERVICES_LIST: &str = include_str!("../constants/services_list.json");

#[tokio::main]
async fn main() {
    todo!("Implementation not implemented yet");
}