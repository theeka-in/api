use crate::errors::{ErrorDto, ServiceError};
use crate::modules::business::BusinessService;
use crate::modules::listing::listing_dto::{
    CreateListingMediaDto, CreateProductListingDto, CreateServiceListingDto, ListingDto,
    ListingMediaDto, UpdateListingDto,
};
use crate::modules::listing::listing_repository::ListingRepository;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug)]
pub struct ListingService {
    repo: ListingRepository,
    business: Arc<BusinessService>,
}

impl ListingService {
    pub fn new(repo: ListingRepository, business_service: Arc<BusinessService>) -> Arc<Self> {
        Arc::new(Self {
            repo,
            business: business_service,
        })
    }

    pub async fn get_all_listings_by_business(
        &self,
        business_id: Uuid,
    ) -> Result<Vec<ListingDto>, ServiceError> {
        self.business.get_business_by_id(business_id).await?;

        let listings = self.repo.find_all_listings_by_business(business_id).await?;
        Ok(listings.into_iter().map(ListingDto::from).collect())
    }

    pub async fn get_listing_by_id_and_business(
        &self,
        business_id: Uuid,
        listing_id: Uuid,
    ) -> Result<ListingDto, ServiceError> {
        self.business.get_business_by_id(business_id).await?;

        let listing = self
            .repo
            .find_listing_by_id_and_business(listing_id, business_id)
            .await?
            .ok_or(ServiceError::NotFound(ErrorDto {
                message: "listing not found".to_owned(),
            }))?;

        Ok(ListingDto::from(listing))
    }

    pub async fn get_listing_by_id_and_business_and_owner(
        &self,
        owner_id: Uuid,
        business_id: Uuid,
        listing_id: Uuid,
    ) -> Result<ListingDto, ServiceError> {
        self.business
            .get_business_by_id_and_owner(business_id, owner_id)
            .await?;

        let listing = self
            .repo
            .find_listing_by_id_and_business_and_owner(listing_id, business_id, owner_id)
            .await?
            .ok_or(ServiceError::NotFound(ErrorDto {
                message: "listing not found".to_owned(),
            }))?;

        Ok(ListingDto::from(listing))
    }

    pub async fn create_product(
        &self,
        owner_id: Uuid,
        business_id: Uuid,
        body: CreateProductListingDto,
    ) -> Result<ListingDto, ServiceError> {
        dbg!(&business_id, &owner_id);

        self.business
            .get_business_by_id_and_owner(business_id, owner_id)
            .await?;

        let listing = self
            .repo
            .create_product_listing(
                business_id,
                body.title,
                body.description,
                body.logo,
                body.price,
                body.stock,
                body.categories,
                body.tags,
            )
            .await?;

        Ok(ListingDto::from(listing))
    }

    pub async fn create_service(
        &self,
        owner_id: Uuid,
        business_id: Uuid,
        body: CreateServiceListingDto,
    ) -> Result<ListingDto, ServiceError> {
        self.business
            .get_business_by_id_and_owner(business_id, owner_id)
            .await?;

        let listing = self
            .repo
            .create_service_listing(
                business_id,
                body.title,
                body.description,
                body.logo,
                body.price,
                body.available,
                body.categories,
                body.tags,
            )
            .await?;

        Ok(ListingDto::from(listing))
    }

    pub async fn update_listing(
        &self,
        owner_id: Uuid,
        business_id: Uuid,
        listing_id: Uuid,
        body: UpdateListingDto,
    ) -> Result<ListingDto, ServiceError> {
        self.get_listing_by_id_and_business_and_owner(owner_id, business_id, listing_id)
            .await?;

        let updated = self
            .repo
            .update_listing(
                listing_id,
                business_id,
                body.title,
                body.description,
                body.logo,
                body.is_active,
            )
            .await?;

        Ok(ListingDto::from(updated))
    }

    pub async fn delete_listing(
        &self,
        owner_id: Uuid,
        business_id: Uuid,
        listing_id: Uuid,
    ) -> Result<(), ServiceError> {
        self.get_listing_by_id_and_business_and_owner(owner_id, business_id, listing_id)
            .await?;

        self.repo.delete_listing(listing_id, business_id).await?;

        Ok(())
    }

    pub async fn get_media(
        &self,
        business_id: Uuid,
        listing_id: Uuid,
    ) -> Result<Vec<ListingMediaDto>, ServiceError> {
        self.get_listing_by_id_and_business(listing_id, business_id)
            .await?;

        let media = self.repo.find_media(listing_id).await?;
        Ok(media.into_iter().map(ListingMediaDto::from).collect())
    }

    pub async fn create_media(
        &self,
        owner_id: Uuid,
        business_id: Uuid,
        listing_id: Uuid,
        body: CreateListingMediaDto,
    ) -> Result<ListingMediaDto, ServiceError> {
        self.get_listing_by_id_and_business_and_owner(owner_id, business_id, listing_id)
            .await?;

        let media = self
            .repo
            .create_media(listing_id, body.media_type, body.url)
            .await?;

        Ok(ListingMediaDto::from(media))
    }

    pub async fn delete_media(
        &self,
        owner_id: Uuid,
        business_id: Uuid,
        listing_id: Uuid,
        media_id: Uuid,
    ) -> Result<(), ServiceError> {
        self.get_listing_by_id_and_business_and_owner(owner_id, business_id, listing_id)
            .await?;

        self.repo.delete_media(media_id, listing_id).await?;

        Ok(())
    }
}
