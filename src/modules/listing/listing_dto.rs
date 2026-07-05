use poem_openapi::Object;
use uuid::Uuid;

use crate::modules::{
    business::BusinessWithOwnerAndAddressDto,
    database::{
        ExploreProductListingEntity, ExploreServiceListingEntity,
        ListingEntity, ListingMediaEntity, ProductListingEntity,
        ServiceListingEntity,
    },
};

#[derive(Debug, Object)]
pub struct ListingDto {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub logo: Option<String>,
    pub is_active: bool,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Object)]
pub struct ProductDto {
    pub id: Uuid,
    pub price: f64,
    pub stock: i32,
}

#[derive(Debug, Object)]
pub struct ServiceDto {
    pub id: Uuid,
    pub price: String,
    pub available: bool,
}

#[derive(Debug, Object)]
pub struct ProductListingDto {
    #[oai(flatten)]
    pub listing: ListingDto,
    pub product: ProductDto,
}

#[derive(Debug, Object)]
pub struct ServiceListingDto {
    #[oai(flatten)]
    pub listing: ListingDto,
    pub service: ServiceDto,
}

#[derive(Debug, Object)]
pub struct ExploreProductListingDto {
    pub product: ProductDto,
    #[oai(flatten)]
    pub listing: ListingDto,
    pub media: Vec<ListingMediaDto>,
    pub business: BusinessWithOwnerAndAddressDto,
}

#[derive(Debug, Object)]
pub struct ExploreServiceListingDto {
    pub service: ServiceDto,
    #[oai(flatten)]
    pub listing: ListingDto,
    pub media: Vec<ListingMediaDto>,
    pub business: BusinessWithOwnerAndAddressDto,
}

#[derive(Debug, Object)]
pub struct CreateProductListingDto {
    #[oai(validator(min_length = 3, max_length = 120))]
    pub title: String,
    pub description: Option<String>,
    pub logo: Option<String>,
    pub price: f64,
    pub stock: i32,
}

#[derive(Debug, Object)]
pub struct CreateServiceListingDto {
    #[oai(validator(min_length = 3, max_length = 120))]
    pub title: String,
    pub description: Option<String>,
    pub logo: Option<String>,
    pub price: String,
    pub available: bool,
    pub service_listing_type_id: Uuid,
}

#[derive(Debug, Object)]
pub struct UpdateProductListingDto {
    #[oai(validator(min_length = 3, max_length = 120))]
    pub title: Option<String>,
    pub description: Option<String>,
    pub logo: Option<String>,
    pub is_active: Option<bool>,
    pub price: Option<f64>,
    pub stock: Option<i32>,
}

#[derive(Debug, Object)]
pub struct UpdateServiceListingDto {
    #[oai(validator(min_length = 3, max_length = 120))]
    pub title: Option<String>,
    pub description: Option<String>,
    pub logo: Option<String>,
    pub is_active: Option<bool>,
    pub price: Option<String>,
    pub available: Option<bool>,
    pub service_listing_type_id: Option<Uuid>,
}

#[derive(Debug, Object)]
pub struct ListingMediaDto {
    pub id: Uuid,
    pub media_type: String,
    pub url: String,
    pub listing_id: Uuid,
}

#[derive(Debug, Object)]
pub struct CreateListingMediaDto {
    pub media_type: String,
    pub url: String,
}

impl From<(ListingEntity, ProductListingEntity)> for ProductListingDto {
    fn from((listing, product): (ListingEntity, ProductListingEntity)) -> Self {
        Self {
            product: ProductDto {
                id: product.id,
                price: product.price,
                stock: product.stock,
            },
            listing: ListingDto {
                id: listing.id,
                title: listing.title,
                description: listing.description,
                logo: listing.logo,
                is_active: listing.is_active,
                created_at: listing.created_at.to_string(),
                updated_at: listing.updated_at.to_string(),
            },
        }
    }
}

impl From<(ListingEntity, ServiceListingEntity)> for ServiceListingDto {
    fn from((listing, service): (ListingEntity, ServiceListingEntity)) -> Self {
        Self {
            service: ServiceDto {
                id: service.id,
                price: service.price,
                available: service.available,
            },
            listing: ListingDto {
                id: listing.id,
                title: listing.title,
                description: listing.description,
                logo: listing.logo,
                is_active: listing.is_active,
                created_at: listing.created_at.to_string(),
                updated_at: listing.updated_at.to_string(),
            },
        }
    }
}

impl From<ExploreProductListingEntity> for ExploreProductListingDto {
    fn from(
        ExploreProductListingEntity {
            business,
            business_address,
            listing,
            listing_media,
            product,
            business_owner,
        }: ExploreProductListingEntity,
    ) -> Self {
        Self {
            product: ProductDto {
                id: product.id,
                price: product.price,
                stock: product.stock,
            },
            listing: ListingDto {
                id: listing.id,
                title: listing.title,
                description: listing.description,
                logo: listing.logo,
                is_active: listing.is_active,
                created_at: listing.created_at.to_string(),
                updated_at: listing.updated_at.to_string(),
            },
            business: (business, business_address, business_owner).into(),
            media: listing_media
                .into_iter()
                .map(ListingMediaDto::from)
                .collect(),
        }
    }
}

impl From<ExploreServiceListingEntity> for ExploreServiceListingDto {
    fn from(
        ExploreServiceListingEntity {
            business,
            business_address,
            listing,
            listing_media,
            service,
            business_owner,
        }: ExploreServiceListingEntity,
    ) -> Self {
        Self {
            service: ServiceDto {
                id: service.id,
                price: service.price,
                available: service.available,
            },
            listing: ListingDto {
                id: listing.id,
                title: listing.title,
                description: listing.description,
                logo: listing.logo,
                is_active: listing.is_active,
                created_at: listing.created_at.to_string(),
                updated_at: listing.updated_at.to_string(),
            },
            business: (business, business_address, business_owner).into(),
            media: listing_media
                .into_iter()
                .map(ListingMediaDto::from)
                .collect(),
        }
    }
}

impl From<ListingMediaEntity> for ListingMediaDto {
    fn from(entity: ListingMediaEntity) -> Self {
        Self {
            id: entity.id,
            media_type: format!("{:?}", entity.media_type).to_lowercase(),
            url: entity.url,
            listing_id: entity.listing_id,
        }
    }
}
