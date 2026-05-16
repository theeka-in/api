use std::env;

use poem::{Server, listener::TcpListener};
use sqlx::postgres::PgPoolOptions;

mod api;
mod errors;
mod modules;
mod utils;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    dotenvy::dotenv().ok();

    let the_port = env::var("PORT").unwrap_or("3000".to_owned());
    let the_address = format!("0.0.0.0:{the_port}");

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pg_pool = PgPoolOptions::new()
        .max_connections(20)
        .connect(&database_url)
        .await
        .expect("Failed to connect to database");

    if std::env::var("ENV").expect("ENV is not defined") == "prod" {
        sqlx::migrate!("./migrations")
            .run(&pg_pool)
            .await
            .expect("Migrations directory was not found");
    }

    let (the_api, _) = api::init(pg_pool, &the_port).await;

    Server::new(TcpListener::bind(the_address))
        .run(the_api)
        .await
}
