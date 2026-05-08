use std::env;

use poem::{Server, listener::TcpListener};

mod app;
mod database;
mod entities;
mod modules;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let the_port = env::var("PORT").unwrap_or("3000".to_owned());
    let the_address = format!("0.0.0.0:{the_port}");
    let the_app = app::init(&the_port).await;

    Server::new(TcpListener::bind(the_address))
        .run(the_app)
        .await
}
