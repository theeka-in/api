use std::sync::Arc;

use sqlx::{Pool, Postgres};
use tokio::sync::Mutex;

use crate::features::{
    analytics::{AnalyticsController, AnalyticsRepository, AnalyticsService},
    auth::{AuthController, AuthRepository, AuthService},
    business::{BusinessController, BusinessRepository, BusinessService},
    embedding::EmbeddingService,
    health::HealthController,
    listing::{ListingController, ListingRepository, ListingService},
    review::{ReviewController, ReviewRepository, ReviewService},
    users::{UsersController, UsersRepository, UsersService},
};

pub mod analytics;
pub mod auth;
pub mod business;
pub mod embedding;
pub mod health;
pub mod listing;
pub mod review;
pub mod users;

pub struct Services {
    pub auth: Arc<AuthService>,
}

pub struct Controllers(
    pub  (
        AuthController,
        UsersController,
        BusinessController,
        ListingController,
        ReviewController,
        AnalyticsController,
        HealthController,
    ),
);

pub fn init(pg_pool: Pool<Postgres>, ollama_url: String) -> (Services, Controllers) {
    let users_repository = UsersRepository::new(pg_pool.clone());
    let auth_repository = AuthRepository::new(pg_pool.clone());
    let business_repository = BusinessRepository::new(pg_pool.clone());
    let listing_repository = ListingRepository::new(pg_pool.clone());
    let review_repository = ReviewRepository::new(pg_pool.clone());
    let analytics_repository = AnalyticsRepository::new(pg_pool);

    let embedding_service = EmbeddingService::new(ollama_url);

    let users_service = UsersService::new(users_repository);
    let auth_service = AuthService::new(auth_repository, users_service.clone());
    let business_service = BusinessService::new(
        business_repository,
        users_service.clone(),
        auth_service.clone(),
    );
    let listing_service = ListingService::new(
        listing_repository,
        business_service.clone(),
        embedding_service.clone(),
    );
    let review_service = ReviewService::new(review_repository);
    let analytics_service = AnalyticsService::new(analytics_repository);

    let health_controller = HealthController::new();

    let auth_controller = AuthController::new(auth_service.clone());
    let users_controller = UsersController::new(users_service);
    let business_controller = BusinessController::new(business_service);
    let listing_controller = ListingController::new(listing_service);
    let review_controller = ReviewController::new(review_service);
    let analytics_controller = AnalyticsController::new(analytics_service);

    let exposed_services = Services { auth: auth_service };

    let exposed_controllers = Controllers((
        auth_controller,
        users_controller,
        business_controller,
        listing_controller,
        review_controller,
        analytics_controller,
        health_controller,
    ));

    (exposed_services, exposed_controllers)
}
