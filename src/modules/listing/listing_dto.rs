use crate::modules::{
    business::{BusinessAddressEntity, BusinessDto, BusinessEntity},
    listing::listing_entity::{
        BusinessListingEntity, ListingMediaEntity, ProductListingEntity, ServiceListingEntity,
    },
};
use poem_openapi::Object;
use uuid::Uuid;

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
pub struct ProductListingDto {
    pub price: f64,
    pub stock: i32,
    pub listing: ListingDto,
}

#[derive(Debug, Object)]
pub struct ServiceListingDto {
    pub price: String,
    pub available: bool,
    pub listing: ListingDto,
}

#[derive(Debug, Object)]
pub struct ExploreListingDto {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub logo: Option<String>,
    pub is_active: bool,
    pub created_at: String,
    pub updated_at: String,
    pub media: Vec<ListingMediaDto>,
    pub business: BusinessDto,
}

#[derive(Debug, Object)]
pub struct ExploreProductListingDto {
    pub price: f64,
    pub stock: i32,
    pub listing: ExploreListingDto,
}

#[derive(Debug, Object)]
pub struct ExploreServiceListingDto {
    pub price: String,
    pub available: bool,
    pub listing: ExploreListingDto,
}

#[derive(Debug, Object)]
pub struct CreateProductListingDto {
    #[oai(validator(min_length = 3, max_length = 120))]
    pub title: String,
    pub description: Option<String>,
    pub logo: Option<String>,
    pub price: f64,
    pub stock: i32,
    pub categories: Option<Vec<String>>,
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Object)]
pub struct CreateServiceListingDto {
    #[oai(validator(min_length = 3, max_length = 120))]
    pub title: String,
    pub description: Option<String>,
    pub logo: Option<String>,
    pub price: String,
    pub available: bool,
    pub categories: Option<Vec<String>>,
    pub tags: Option<Vec<String>>,
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

impl From<(BusinessListingEntity, ProductListingEntity)> for ProductListingDto {
    fn from((listing, product): (BusinessListingEntity, ProductListingEntity)) -> Self {
        Self {
            price: product.price,
            stock: product.stock,
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

impl From<(BusinessListingEntity, ServiceListingEntity)> for ServiceListingDto {
    fn from((listing, service): (BusinessListingEntity, ServiceListingEntity)) -> Self {
        Self {
            price: service.price,
            available: service.available,
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

impl
    From<(
        BusinessEntity,
        BusinessAddressEntity,
        BusinessListingEntity,
        Vec<ListingMediaEntity>,
        ProductListingEntity,
    )> for ExploreProductListingDto
{
    fn from(
        (business, address, listing, media_list, product): (
            BusinessEntity,
            BusinessAddressEntity,
            BusinessListingEntity,
            Vec<ListingMediaEntity>,
            ProductListingEntity,
        ),
    ) -> Self {
        Self {
            price: product.price,
            stock: product.stock,
            listing: ExploreListingDto {
                id: listing.id,
                title: listing.title,
                description: listing.description,
                logo: listing.logo,
                is_active: listing.is_active,
                created_at: listing.created_at.to_string(),
                updated_at: listing.updated_at.to_string(),
                media: media_list.into_iter().map(ListingMediaDto::from).collect(),
                business: BusinessDto {
                    id: business.id,
                    phone_number: business.phone_number,
                    is_closed: business.is_closed,
                    title: business.title,
                    logo: business.logo,
                    description: business.description,
                    created_at: business.created_at.to_string(),
                    owner_id: business.owner_id,
                    address: Some(address.into()),
                },
            },
        }
    }
}

impl
    From<(
        BusinessEntity,
        BusinessAddressEntity,
        BusinessListingEntity,
        Vec<ListingMediaEntity>,
        ServiceListingEntity,
    )> for ExploreServiceListingDto
{
    fn from(
        (business, address, listing, media_list, service): (
            BusinessEntity,
            BusinessAddressEntity,
            BusinessListingEntity,
            Vec<ListingMediaEntity>,
            ServiceListingEntity,
        ),
    ) -> Self {
        Self {
            price: service.price,
            available: service.available,
            listing: ExploreListingDto {
                id: listing.id,
                title: listing.title,
                description: listing.description,
                logo: listing.logo,
                is_active: listing.is_active,
                created_at: listing.created_at.to_string(),
                updated_at: listing.updated_at.to_string(),
                media: media_list.into_iter().map(ListingMediaDto::from).collect(),
                business: BusinessDto {
                    id: business.id,
                    phone_number: business.phone_number,
                    is_closed: business.is_closed,
                    title: business.title,
                    logo: business.logo,
                    description: business.description,
                    created_at: business.created_at.to_string(),
                    owner_id: business.owner_id,
                    address: Some(address.into()),
                },
            },
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
