use poem::{Route, listener::TcpListener};
use poem_openapi::{OpenApi, OpenApiService, param::Query, payload::PlainText};

struct Api;

#[OpenApi]
impl Api {
    #[oai(path = "/hello", method = "get")]
    async fn index(
        &self,
        #[oai(validator(min_length = 2, max_length = 50))] name: Query<Option<String>>,
    ) -> PlainText<String> {
        match name.0 {
            Some(name) => PlainText(format!("hello, {}!", name)),
            None => PlainText("hello, no name".to_owned()),
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let api_service =
        OpenApiService::new(Api, "Hello World", "1.0").server("http://localhost:3000/api");

    let ui = api_service.scalar();

    let app = Route::new().nest("/api", api_service).nest("/", ui);

    poem::Server::new(TcpListener::bind("0.0.0.0:3000"))
        .run(app)
        .await
}
