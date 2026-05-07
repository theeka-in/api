use poem::Route;
use poem_openapi::OpenApiService;

use crate::{
    database::get_connection_url,
    modules::{
        auth::{AuthController, AuthService},
        database::DatabaseService,
        users::{UsersController, UsersService},
    },
};

pub async fn init(port: &str) -> Route {
    let database_service = DatabaseService::new(get_connection_url()).await;
    let users_service = UsersService::new();
    let auth_service = AuthService::new(database_service, users_service.clone());

    let the_api = OpenApiService::new(
        (
            UsersController::new(users_service),
            AuthController::new(auth_service),
        ),
        "Theeka",
        "1.0",
    )
    .server(format!("http://localhost:{port}/api"));

    let the_json_spec = the_api.spec_endpoint();
    let the_yaml_spec = the_api.spec_endpoint_yaml();
    let the_ui = the_api.scalar();

    Route::new()
        .nest("/api", the_api)
        .nest("/", the_ui)
        .nest("/openapi.json", the_json_spec)
        .nest("/openapi.yaml", the_yaml_spec)
}
