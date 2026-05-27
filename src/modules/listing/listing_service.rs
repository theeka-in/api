use crate::errors::{ErrorDto, ServiceError};
use crate::modules::business::BusinessService;
use crate::modules::listing::listing_dto::{
    CreateListingMediaDto, CreateProductListingDto, CreateServiceListingDto, ListingMediaDto,
    ProductListingDto, ServiceListingDto, UpdateProductListingDto, UpdateServiceListingDto,
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

    pub async fn explore_products_listings_nearby(
        &self,
        latitude: f64,
        longitude: f64,
    ) -> Result<Vec<ProductListingDto>, ServiceError> {
        let listings = self
            .repo
            .explore_products_listings_nearby(latitude, longitude)
            .await?;
        
        Ok(listings.into_iter().map(ProductListingDto::from).collect())
    }
    
    pub async fn explore_services_listings_nearby(
        &self,
        latitude: f64,
        longitude: f64,
    ) -> Result<Vec<ServiceListingDto>, ServiceError> {
        let listings = self
            .repo
            .explore_services_listings_nearby(latitude, longitude)
            .await?;

        Ok(listings.into_iter().map(ServiceListingDto::from).collect())
    }

    pub async fn get_all_product_listings_by_business(
        &self,
        business_id: Uuid,
    ) -> Result<Vec<ProductListingDto>, ServiceError> {
        self.business.get_business_by_id(business_id).await?;
        let listings = self
            .repo
            .find_all_products_listings_by_business(business_id)
            .await?;
        Ok(listings.into_iter().map(ProductListingDto::from).collect())
    }

    pub async fn get_product_listing_by_id_and_business(
        &self,
        business_id: Uuid,
        listing_id: Uuid,
    ) -> Result<ProductListingDto, ServiceError> {
        self.business.get_business_by_id(business_id).await?;
        let listing = self
            .repo
            .find_product_listing_by_id_and_business(listing_id, business_id)
            .await?
            .ok_or(ServiceError::NotFound(ErrorDto {
                message: "product listing not found".to_owned(),
            }))?;
        Ok(ProductListingDto::from(listing))
    }

    pub async fn get_product_listing_by_id_and_business_and_owner(
        &self,
        owner_id: Uuid,
        business_id: Uuid,
        listing_id: Uuid,
    ) -> Result<ProductListingDto, ServiceError> {
        self.business
            .get_business_by_id_and_owner(business_id, owner_id)
            .await?;
        let listing = self
            .repo
            .find_product_listing_by_id_and_business_and_owner(listing_id, business_id, owner_id)
            .await?
            .ok_or(ServiceError::NotFound(ErrorDto {
                message: "product listing not found".to_owned(),
            }))?;
        Ok(ProductListingDto::from(listing))
    }

    pub async fn create_product(
        &self,
        owner_id: Uuid,
        business_id: Uuid,
        body: CreateProductListingDto,
    ) -> Result<ProductListingDto, ServiceError> {
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
        Ok(ProductListingDto::from(listing))
    }

    pub async fn update_product(
        &self,
        owner_id: Uuid,
        business_id: Uuid,
        listing_id: Uuid,
        body: UpdateProductListingDto,
    ) -> Result<ProductListingDto, ServiceError> {
        self.get_product_listing_by_id_and_business_and_owner(owner_id, business_id, listing_id)
            .await?;
        let updated = self
            .repo
            .update_product_listing(
                listing_id,
                business_id,
                body.title,
                body.description,
                body.logo,
                body.is_active,
                body.price,
                body.stock,
            )
            .await?;
        Ok(ProductListingDto::from(updated))
    }

    pub async fn delete_product(
        &self,
        owner_id: Uuid,
        business_id: Uuid,
        listing_id: Uuid,
    ) -> Result<(), ServiceError> {
        self.get_product_listing_by_id_and_business_and_owner(owner_id, business_id, listing_id)
            .await?;
        self.repo
            .delete_product_listing(listing_id, business_id)
            .await?;
        Ok(())
    }

    pub async fn get_all_service_listings_by_business(
        &self,
        business_id: Uuid,
    ) -> Result<Vec<ServiceListingDto>, ServiceError> {
        self.business.get_business_by_id(business_id).await?;
        let listings = self
            .repo
            .find_all_services_listings_by_business(business_id)
            .await?;
        Ok(listings.into_iter().map(ServiceListingDto::from).collect())
    }

    pub async fn get_service_listing_by_id_and_business(
        &self,
        business_id: Uuid,
        listing_id: Uuid,
    ) -> Result<ServiceListingDto, ServiceError> {
        self.business.get_business_by_id(business_id).await?;
        let listing = self
            .repo
            .find_service_listing_by_id_and_business(listing_id, business_id)
            .await?
            .ok_or(ServiceError::NotFound(ErrorDto {
                message: "service listing not found".to_owned(),
            }))?;
        Ok(ServiceListingDto::from(listing))
    }

    pub async fn get_service_listing_by_id_and_business_and_owner(
        &self,
        owner_id: Uuid,
        business_id: Uuid,
        listing_id: Uuid,
    ) -> Result<ServiceListingDto, ServiceError> {
        self.business
            .get_business_by_id_and_owner(business_id, owner_id)
            .await?;
        let listing = self
            .repo
            .find_service_listing_by_id_and_business_and_owner(listing_id, business_id, owner_id)
            .await?
            .ok_or(ServiceError::NotFound(ErrorDto {
                message: "service listing not found".to_owned(),
            }))?;
        Ok(ServiceListingDto::from(listing))
    }

    pub async fn create_service(
        &self,
        owner_id: Uuid,
        business_id: Uuid,
        body: CreateServiceListingDto,
    ) -> Result<ServiceListingDto, ServiceError> {
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
        Ok(ServiceListingDto::from(listing))
    }

    pub async fn update_service(
        &self,
        owner_id: Uuid,
        business_id: Uuid,
        listing_id: Uuid,
        body: UpdateServiceListingDto,
    ) -> Result<ServiceListingDto, ServiceError> {
        self.get_service_listing_by_id_and_business_and_owner(owner_id, business_id, listing_id)
            .await?;
        let updated = self
            .repo
            .update_service_listing(
                listing_id,
                business_id,
                body.title,
                body.description,
                body.logo,
                body.is_active,
                body.price,
                body.available,
            )
            .await?;
        Ok(ServiceListingDto::from(updated))
    }

    pub async fn delete_service(
        &self,
        owner_id: Uuid,
        business_id: Uuid,
        listing_id: Uuid,
    ) -> Result<(), ServiceError> {
        self.get_service_listing_by_id_and_business_and_owner(owner_id, business_id, listing_id)
            .await?;
        self.repo
            .delete_service_listing(listing_id, business_id)
            .await?;
        Ok(())
    }

    pub async fn get_media(
        &self,
        business_id: Uuid,
        listing_id: Uuid,
    ) -> Result<Vec<ListingMediaDto>, ServiceError> {
        let exists = self
            .repo
            .find_product_listing_by_id_and_business(listing_id, business_id)
            .await?;
        if exists.is_none() {
            self.repo
                .find_service_listing_by_id_and_business(listing_id, business_id)
                .await?
                .ok_or(ServiceError::NotFound(ErrorDto {
                    message: "listing not found".to_owned(),
                }))?;
        }
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
        let exists = self
            .repo
            .find_product_listing_by_id_and_business_and_owner(listing_id, business_id, owner_id)
            .await?;
        if exists.is_none() {
            self.repo
                .find_service_listing_by_id_and_business_and_owner(
                    listing_id,
                    business_id,
                    owner_id,
                )
                .await?
                .ok_or(ServiceError::NotFound(ErrorDto {
                    message: "listing not found".to_owned(),
                }))?;
        }
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
        let exists = self
            .repo
            .find_product_listing_by_id_and_business_and_owner(listing_id, business_id, owner_id)
            .await?;
        if exists.is_none() {
            self.repo
                .find_service_listing_by_id_and_business_and_owner(
                    listing_id,
                    business_id,
                    owner_id,
                )
                .await?
                .ok_or(ServiceError::NotFound(ErrorDto {
                    message: "listing not found".to_owned(),
                }))?;
        }
        self.repo.delete_media(media_id, listing_id).await?;
        Ok(())
    }
}
