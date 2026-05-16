use poem::{EndpointExt, Route};
use poem_openapi::OpenApiService;
use sqlx::{Pool, Postgres};

use crate::{
    middleware::ErrorHandlerMiddleware,
    modules::{
        analytics::{AnalyticsController, AnalyticsRepository, AnalyticsService},
        auth::{AuthController, AuthRepository, AuthService},
        business::{BusinessController, BusinessRepository, BusinessService},
        health::HealthController,
        listing::{ListingController, ListingRepository, ListingService},
        review::{ReviewController, ReviewRepository, ReviewService},
        users::{UsersController, UsersRepository, UsersService},
    },
};

pub async fn init(pg_pool: Pool<Postgres>, port: &str) -> (Route, String) {
    let users_repository = UsersRepository::new(pg_pool.clone());
    let auth_repository = AuthRepository::new(pg_pool.clone());
    let business_repository = BusinessRepository::new(pg_pool.clone());
    let listing_repository = ListingRepository::new(pg_pool.clone());
    let review_repository = ReviewRepository::new(pg_pool.clone());
    let analytics_repository = AnalyticsRepository::new(pg_pool);

    let users_service = UsersService::new(users_repository);
    let auth_service = AuthService::new(auth_repository, users_service.clone());
    let business_service = BusinessService::new(business_repository);
    let listing_service = ListingService::new(listing_repository);
    let review_service = ReviewService::new(review_repository);
    let analytics_service = AnalyticsService::new(analytics_repository);

    let auth_controller = AuthController::new(auth_service.clone());
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

    let json_spec_string = the_api.spec();

    (
        Route::new()
            .nest(
                "/api",
                the_api
                    // exception: this is for the bearer auth to work
                    .data(auth_service)
                    .with(ErrorHandlerMiddleware),
            )
            .nest("/", the_ui)
            .nest("/openapi.json", the_json_spec)
            .nest("/openapi.yaml", the_yaml_spec),
        json_spec_string,
    )
}
