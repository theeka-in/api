use std::sync::Arc;

use sqlx::{Pool, Postgres};

use crate::modules::{
    analytics::{AnalyticsController, AnalyticsService},
    auth::{AuthController, AuthService},
    business::{BusinessController, BusinessService},
    database::DatabaseService,
    health::HealthController,
    listing::{ListingController, ListingService},
    location::{LocationController, LocationService},
    review::{ReviewController, ReviewService},
    users::{UsersController, UsersService},
};

pub mod analytics;
pub mod auth;
pub mod business;
pub mod database;
pub mod health;
pub mod listing;
pub mod location;
pub mod review;
pub mod users;

pub struct ExposedServices {
    pub auth: Arc<AuthService>,
}

pub struct ExposedControllers(
    pub  (
        AuthController,
        UsersController,
        BusinessController,
        ListingController,
        ReviewController,
        AnalyticsController,
        HealthController,
        LocationController,
    ),
);

pub fn init(pg_pool: Pool<Postgres>, ollama_url: String) -> (ExposedServices, ExposedControllers) {
    let database_service = DatabaseService::new(pg_pool);

    let users_service = UsersService::new(database_service.clone());
    let auth_service = AuthService::new(database_service.clone(), users_service.clone());
    let business_service = BusinessService::new(
        database_service.clone(),
        users_service.clone(),
        auth_service.clone(),
    );
    let listing_service = ListingService::new(
        database_service.clone(),
        business_service.clone(),
    );
    let review_service = ReviewService::new(database_service.clone());
    let analytics_service = AnalyticsService::new(database_service);
    let location_service = LocationService::new();

    let health_controller = HealthController::new();

    let auth_controller = AuthController::new(auth_service.clone());
    let users_controller = UsersController::new(users_service);
    let business_controller = BusinessController::new(business_service);
    let listing_controller = ListingController::new(listing_service);
    let review_controller = ReviewController::new(review_service);
    let analytics_controller = AnalyticsController::new(analytics_service);
    let location_controller = LocationController::new(location_service);

    let exposed_services = ExposedServices { auth: auth_service };

    let exposed_controllers = ExposedControllers((
        auth_controller,
        users_controller,
        business_controller,
        listing_controller,
        review_controller,
        analytics_controller,
        health_controller,
        location_controller,
    ));

    (exposed_services, exposed_controllers)
}
