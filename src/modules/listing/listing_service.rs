use crate::modules::business::BusinessService;
use crate::modules::database::DatabaseService;
use crate::modules::embedding::EmbeddingService;
use crate::modules::listing::listing_dto::{
    CreateListingMediaDto, CreateProductListingDto, CreateServiceListingDto, ListingMediaDto,
    ProductListingDto, ServiceListingDto, UpdateProductListingDto, UpdateServiceListingDto,
};
use crate::modules::listing::{ExploreProductListingDto, ExploreServiceListingDto};
use crate::shared::errors::{ErrorDto, ServiceError};
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;

#[derive(Debug)]
pub struct ListingService {
    db: Arc<DatabaseService>,
    business: Arc<BusinessService>,
    embedding: Arc<Mutex<EmbeddingService>>,
}

impl ListingService {
    pub fn new(
        db: Arc<DatabaseService>,
        business_service: Arc<BusinessService>,
        embedding_service: Arc<Mutex<EmbeddingService>>,
    ) -> Arc<Self> {
        Arc::new(Self {
            db,
            business: business_service,
            embedding: embedding_service,
        })
    }

    fn embed_listing_properties(title: &str, description: Option<&str>, price: &str) -> String {
        let mut return_value = format!("Title: {}\nPrice: {}", title, price);

        if let Some(desc) = description {
            return_value.push_str(&format!("\nDescription: {}", desc));
        }

        return_value
    }

    pub async fn explore_products_listings_nearby(
        &self,
        latitude: f64,
        longitude: f64,
        query: String,
    ) -> Result<Vec<ExploreProductListingDto>, ServiceError> {
        let query_embedding = self.embedding.lock().await.embed(query).await?;

        let listings = self
            .db
            .business_listing
            .explore_products_nearby(latitude, longitude, query_embedding)
            .await?;

        Ok(listings
            .into_iter()
            .map(ExploreProductListingDto::from)
            .collect())
    }

    pub async fn explore_services_listings_nearby(
        &self,
        latitude: f64,
        longitude: f64,
        query: String,
    ) -> Result<Vec<ExploreServiceListingDto>, ServiceError> {
        let query_embedding = self.embedding.lock().await.embed(query).await?;

        let listings = self
            .db
            .business_listing
            .explore_services_nearby(latitude, longitude, query_embedding)
            .await?;

        Ok(listings
            .into_iter()
            .map(ExploreServiceListingDto::from)
            .collect())
    }

    pub async fn get_all_product_listings_by_business(
        &self,
        business_id: Uuid,
    ) -> Result<Vec<ProductListingDto>, ServiceError> {
        self.business.get_business_by_id(business_id).await?;

        let listings = self
            .db
            .product_listing
            .find_all_by_business(business_id)
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
            .db
            .product_listing
            .find_by_id_and_business(listing_id, business_id)
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
            .db
            .product_listing
            .find_by_id_and_business_and_owner(listing_id, business_id, owner_id)
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

        let embedding = self
            .embedding
            .lock()
            .await
            .embed(Self::embed_listing_properties(
                &body.title,
                body.description.as_deref(),
                &body.price.to_string(),
            ))
            .await?;

        let listing = self
            .db
            .product_listing
            .create(
                business_id,
                body.title,
                body.description,
                body.logo,
                body.price,
                body.stock,
                body.categories,
                body.tags,
                embedding,
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
        let prev = self
            .get_product_listing_by_id_and_business_and_owner(owner_id, business_id, listing_id)
            .await?;

        let mut embedding: Option<pgvector::Vector> = None;

        if body.title.is_some() || body.description.is_some() || body.price.is_some() {
            embedding = Some(
                self.embedding
                    .lock()
                    .await
                    .embed(Self::embed_listing_properties(
                        body.title.as_deref().unwrap_or(&prev.listing.title),
                        body.description
                            .as_deref()
                            .or(prev.listing.description.as_deref()),
                        &body.price.unwrap_or(prev.price).to_string(),
                    ))
                    .await?,
            );
        }

        let updated = self
            .db
            .product_listing
            .update(
                listing_id,
                business_id,
                body.title,
                body.description,
                body.logo,
                body.is_active,
                body.price,
                body.stock,
                embedding,
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

        self.db.product_listing.delete(listing_id, business_id).await?;

        Ok(())
    }

    pub async fn get_all_service_listings_by_business(
        &self,
        business_id: Uuid,
    ) -> Result<Vec<ServiceListingDto>, ServiceError> {
        self.business.get_business_by_id(business_id).await?;

        let listings = self
            .db
            .service_listing
            .find_all_by_business(business_id)
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
            .db
            .service_listing
            .find_by_id_and_business(listing_id, business_id)
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
            .db
            .service_listing
            .find_by_id_and_business_and_owner(listing_id, business_id, owner_id)
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

        let embedding = self
            .embedding
            .lock()
            .await
            .embed(Self::embed_listing_properties(
                &body.title,
                body.description.as_deref(),
                &body.price,
            ))
            .await?;

        let listing = self
            .db
            .service_listing
            .create(
                business_id,
                body.title,
                body.description,
                body.logo,
                body.price,
                body.available,
                body.categories,
                body.tags,
                embedding,
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
        let prev = self
            .get_service_listing_by_id_and_business_and_owner(owner_id, business_id, listing_id)
            .await?;

        let mut embedding: Option<pgvector::Vector> = None;

        if body.title.is_some() || body.description.is_some() || body.price.is_some() {
            embedding = Some(
                self.embedding
                    .lock()
                    .await
                    .embed(Self::embed_listing_properties(
                        body.title.as_deref().unwrap_or(&prev.listing.title),
                        body.description
                            .as_deref()
                            .or(prev.listing.description.as_deref()),
                        body.price.as_deref().unwrap_or(&prev.price),
                    ))
                    .await?,
            );
        }

        let updated = self
            .db
            .service_listing
            .update(
                listing_id,
                business_id,
                body.title,
                body.description,
                body.logo,
                body.is_active,
                body.price,
                body.available,
                embedding,
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

        self.db.service_listing.delete(listing_id, business_id).await?;

        Ok(())
    }

    pub async fn get_media(
        &self,
        business_id: Uuid,
        listing_id: Uuid,
    ) -> Result<Vec<ListingMediaDto>, ServiceError> {
        let exists = self
            .db
            .product_listing
            .find_by_id_and_business(listing_id, business_id)
            .await?;

        if exists.is_none() {
            self.db
                .service_listing
                .find_by_id_and_business(listing_id, business_id)
                .await?
                .ok_or(ServiceError::NotFound(ErrorDto {
                    message: "listing not found".to_owned(),
                }))?;
        }

        let media = self.db.listing_media.find_all(listing_id).await?;

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
            .db
            .product_listing
            .find_by_id_and_business_and_owner(listing_id, business_id, owner_id)
            .await?;

        if exists.is_none() {
            self.db
                .service_listing
                .find_by_id_and_business_and_owner(listing_id, business_id, owner_id)
                .await?
                .ok_or(ServiceError::NotFound(ErrorDto {
                    message: "listing not found".to_owned(),
                }))?;
        }

        let media = self
            .db
            .listing_media
            .create(listing_id, body.media_type, body.url)
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
            .db
            .product_listing
            .find_by_id_and_business_and_owner(listing_id, business_id, owner_id)
            .await?;

        if exists.is_none() {
            self.db
                .service_listing
                .find_by_id_and_business_and_owner(listing_id, business_id, owner_id)
                .await?
                .ok_or(ServiceError::NotFound(ErrorDto {
                    message: "listing not found".to_owned(),
                }))?;
        }

        self.db.listing_media.delete(media_id, listing_id).await?;

        Ok(())
    }
}