use std::env;

use sqlx::postgres::PgPoolOptions;

use theeka_api::api;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let the_port = env::var("PORT").unwrap_or("404".to_owned());
    let pg_pool = PgPoolOptions::new()
        .max_connections(1)
        .connect_lazy("postgres://postgres:password@localhost/fake")
        .unwrap();

    let (_, spec) = api::init(pg_pool, &the_port).await;

    print!("{spec}")
}
