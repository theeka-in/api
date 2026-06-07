use std::sync::Arc;

use sqlx::PgPool;

use crate::modules::database::{
    AccountRepo, BusinessAddressRepo, BusinessHourRepo, BusinessListingRepo, BusinessMediaRepo,
    BusinessRepo, ListingMediaRepo, ProductListingRepo, ServiceListingRepo, SessionRepo,
    UserAddressRepo, UserRepo,
};

#[derive(Debug)]
pub struct DatabaseService {
    pg: PgPool,
    pub account: AccountRepo,
    pub session: SessionRepo,
    pub user: UserRepo,
    pub user_address: UserAddressRepo,
    pub business: BusinessRepo,
    pub business_address: BusinessAddressRepo,
    pub business_hour: BusinessHourRepo,
    pub business_media: BusinessMediaRepo,
    pub business_listing: BusinessListingRepo,
    pub product_listing: ProductListingRepo,
    pub service_listing: ServiceListingRepo,
    pub listing_media: ListingMediaRepo,
}

impl DatabaseService {
    pub fn new(pg: PgPool) -> Arc<Self> {
        let account = AccountRepo::new(pg.clone());
        let session = SessionRepo::new(pg.clone());
        let user = UserRepo::new(pg.clone());
        let user_address = UserAddressRepo::new(pg.clone());
        let business = BusinessRepo::new(pg.clone());
        let business_address = BusinessAddressRepo::new(pg.clone());
        let business_hour = BusinessHourRepo::new(pg.clone());
        let business_media = BusinessMediaRepo::new(pg.clone());
        let business_listing = BusinessListingRepo::new(pg.clone());
        let product_listing = ProductListingRepo::new(pg.clone());
        let service_listing = ServiceListingRepo::new(pg.clone());
        let listing_media = ListingMediaRepo::new(pg.clone());

        Arc::new(Self {
            pg,
            account,
            session,
            user,
            user_address,
            business,
            business_address,
            business_hour,
            business_media,
            business_listing,
            product_listing,
            service_listing,
            listing_media,
        })
    }
}
