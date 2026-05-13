use poem::Route;
use poem_openapi::OpenApiService;
use sqlx::postgres::PgPoolOptions;

use crate::modules::{
    analytics::{AnalyticsController, AnalyticsRepository, AnalyticsService},
    auth::{AuthController, AuthRepository, AuthService},
    business::{BusinessController, BusinessRepository, BusinessService},
    health::HealthController,
    listing::{ListingController, ListingRepository, ListingService},
    review::{ReviewController, ReviewRepository, ReviewService},
    users::{UsersController, UsersRepository, UsersService},
};

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

    let auth_service = AuthService::new(AuthRepository::new(pg_pool.clone()));
    let users_service = UsersService::new(UsersRepository::new(pg_pool.clone()));
    let business_service = BusinessService::new(BusinessRepository::new(pg_pool.clone()));
    let listing_service = ListingService::new(ListingRepository::new(pg_pool.clone()));
    let review_service = ReviewService::new(ReviewRepository::new(pg_pool.clone()));
    let analytics_service = AnalyticsService::new(AnalyticsRepository::new(pg_pool));

    let auth_controller = AuthController::new(auth_service);
    let users_controller = UsersController::new(users_service);
    let business_controller = BusinessController::new(business_service);
    let listing_controller = ListingController::new(listing_service);
    let review_controller = ReviewController::new(review_service);
    let analytics_controller = AnalyticsController::new(analytics_service);
    let health_controller = HealthController::new();

    let the_api = OpenApiService::new(
        (
            auth_controller,
            users_controller,
            business_controller,
            listing_controller,
            review_controller,
            analytics_controller,
            health_controller,
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
