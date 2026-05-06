use poem::Route;
use poem_openapi::OpenApiService;

use crate::modules::{
    auth::{AuthController, AuthService},
    users::{UsersController, UsersService},
};

pub fn init() -> Route {
    let users_service = UsersService::new();
    let auth_service = AuthService::new(users_service.clone());

    let the_api = OpenApiService::new(
        (
            UsersController::new(users_service),
            AuthController::new(auth_service),
        ),
        "Theeka",
        "1.0",
    )
    .server("http://localhost:3000/api");

    let the_json_spec = the_api.spec_endpoint();
    let the_yaml_spec = the_api.spec_endpoint_yaml();
    let the_ui = the_api.scalar();

    Route::new()
        .nest("/api", the_api)
        .nest("/", the_ui)
        .nest("/openapi.json", the_json_spec)
        .nest("/openapi.yaml", the_yaml_spec)
}
