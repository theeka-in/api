use crate::modules::{
    auth::{AuthController, AuthService},
    users::{UsersController, UsersService},
};
use poem::Route;
use poem_openapi::OpenApiService;
use sqlx::postgres::PgPoolOptions;

pub async fn init(port: &str) -> Route {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pg_pool = PgPoolOptions::new()
        .max_connections(20)
        .connect(&database_url)
        .await
        .expect("Failed to connect to database");

    if std::env::var("ENV").expect("ENV is not defined") != "dev" {
        sqlx::migrate!("./migrations")
            .run(&pg_pool)
            .await
            .expect("Migrations directory was not found");
    }

    let users_service = UsersService::new();
    let auth_service = AuthService::new(users_service.clone(), pg_pool.clone());

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
