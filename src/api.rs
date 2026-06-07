use poem::{EndpointExt, Route};
use poem_openapi::OpenApiService;
use sqlx::{Pool, Postgres};

use crate::{modules, shared::middleware::ErrorHandlerMiddleware};

pub async fn init(pg_pool: Pool<Postgres>, port: &str, ollama_url: String) -> (Route, String) {
    let (services, controllers) = modules::init(pg_pool, ollama_url);

    let the_api = OpenApiService::new(controllers.0, "Theeka", "1.0")
        .server(format!("http://localhost:{port}/api"));

    let the_json_spec = the_api.spec_endpoint();
    let the_yaml_spec = the_api.spec_endpoint_yaml();
    let the_ui = the_api.scalar();

    let json_spec_string = the_api.spec();

    (
        Route::new()
            .nest(
                "/api",
                the_api
                    // exception: this is for the guards to work
                    .data(services.auth)
                    .with(ErrorHandlerMiddleware),
            )
            .nest("/", the_ui)
            .nest("/openapi.json", the_json_spec)
            .nest("/openapi.yaml", the_yaml_spec),
        json_spec_string,
    )
}
