use poem::{Server, listener::TcpListener};

mod app;
mod modules;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let the_app = app::init();
    const THE_ADDRESS: &'static str = "0.0.0.0:3000";

    Server::new(TcpListener::bind(THE_ADDRESS))
        .run(the_app)
        .await
}
