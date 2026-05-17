use crate::errors::{ErrorDto, ServiceError};
use crate::modules::business::business_dto::{
    BusinessDto, BusinessHourDto, BusinessMediaDto, CreateBusinessDto, CreateBusinessHourDto,
    CreateBusinessMediaDto, UpdateBusinessDto, UpdateBusinessHourDto,
};
use crate::modules::business::business_repository::BusinessRepository;
use crate::modules::users::UsersService;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug)]
pub struct BusinessService {
    repo: BusinessRepository,
    users: Arc<UsersService>,
}

impl BusinessService {
    pub fn new(repo: BusinessRepository, users_service: Arc<UsersService>) -> Arc<Self> {
        Arc::new(Self {
            repo,
            users: users_service,
        })
    }

    async fn get_owner_id(&self, account_id: Uuid) -> Result<Uuid, ServiceError> {
        let user = self.users.get(account_id).await?;
        Ok(user.id)
    }

    pub async fn get_nearby(
        &self,
        latitude: f64,
        longitude: f64,
    ) -> Result<Vec<BusinessDto>, ServiceError> {
        todo!()
    }

    pub async fn get_by_id(&self, id: Uuid) -> Result<BusinessDto, ServiceError> {
        let business = self
            .repo
            .find_by_id(id)
            .await?
            .ok_or(ServiceError::NotFound(ErrorDto {
                message: "business not found".to_owned(),
            }))?;

        Ok(BusinessDto::from(business))
    }

    pub async fn get_all_by_owner(
        &self,
        account_id: Uuid,
    ) -> Result<Vec<BusinessDto>, ServiceError> {
        let owner_id = self.get_owner_id(account_id).await?;
        let businesses = self.repo.find_all_by_owner(owner_id).await?;

        Ok(businesses.into_iter().map(BusinessDto::from).collect())
    }

    pub async fn create(
        &self,
        account_id: Uuid,
        account_phone: i64,
        body: CreateBusinessDto,
    ) -> Result<BusinessDto, ServiceError> {
        let user = self.users.get(account_id).await?;

        let phone_number = body.phone_number.unwrap_or(account_phone);

        let business = self
            .repo
            .create(
                user.id,
                phone_number,
                body.title,
                body.logo,
                body.description,
            )
            .await?;

        Ok(BusinessDto::from(business))
    }

    pub async fn update(
        &self,
        id: Uuid,
        account_id: Uuid,
        body: UpdateBusinessDto,
    ) -> Result<BusinessDto, ServiceError> {
        let owner_id = self.get_owner_id(account_id).await?;

        let business = self
            .repo
            .find_by_id(id)
            .await?
            .ok_or(ServiceError::NotFound(ErrorDto {
                message: "business not found".to_owned(),
            }))?;

        if business.owner_id != owner_id {
            return Err(ServiceError::Forbidden(ErrorDto {
                message: "you do not own this business".to_owned(),
            }));
        }

        let updated = self
            .repo
            .update(
                id,
                body.phone_number,
                body.title,
                body.logo,
                body.description,
                body.is_closed,
            )
            .await?;

        Ok(BusinessDto::from(updated))
    }

    pub async fn delete(&self, id: Uuid, account_id: Uuid) -> Result<(), ServiceError> {
        let owner_id = self.get_owner_id(account_id).await?;

        let business = self
            .repo
            .find_by_id(id)
            .await?
            .ok_or(ServiceError::NotFound(ErrorDto {
                message: "business not found".to_owned(),
            }))?;

        if business.owner_id != owner_id {
            return Err(ServiceError::Forbidden(ErrorDto {
                message: "you do not own this business".to_owned(),
            }));
        }

        self.repo.delete(id).await?;

        Ok(())
    }

    pub async fn get_hours(&self, business_id: Uuid) -> Result<Vec<BusinessHourDto>, ServiceError> {
        let hours = self.repo.find_hours(business_id).await?;

        Ok(hours.into_iter().map(BusinessHourDto::from).collect())
    }

    pub async fn create_hour(
        &self,
        business_id: Uuid,
        account_id: Uuid,
        body: CreateBusinessHourDto,
    ) -> Result<BusinessHourDto, ServiceError> {
        let owner_id = self.get_owner_id(account_id).await?;

        let business = self
            .repo
            .find_by_id(business_id)
            .await?
            .ok_or(ServiceError::NotFound(ErrorDto {
                message: "business not found".to_owned(),
            }))?;

        if business.owner_id != owner_id {
            return Err(ServiceError::Forbidden(ErrorDto {
                message: "you do not own this business".to_owned(),
            }));
        }

        let hour = self
            .repo
            .create_hour(
                business_id,
                body.day,
                body.hours_type,
                body.open_time,
                body.close_time,
            )
            .await?;

        Ok(BusinessHourDto::from(hour))
    }

    pub async fn update_hour(
        &self,
        business_id: Uuid,
        account_id: Uuid,
        day: String,
        body: UpdateBusinessHourDto,
    ) -> Result<BusinessHourDto, ServiceError> {
        let owner_id = self.get_owner_id(account_id).await?;

        let business = self
            .repo
            .find_by_id(business_id)
            .await?
            .ok_or(ServiceError::NotFound(ErrorDto {
                message: "business not found".to_owned(),
            }))?;

        if business.owner_id != owner_id {
            return Err(ServiceError::Forbidden(ErrorDto {
                message: "you do not own this business".to_owned(),
            }));
        }

        let hour = self
            .repo
            .update_hour(
                business_id,
                day,
                body.hours_type,
                body.open_time,
                body.close_time,
            )
            .await?;

        Ok(BusinessHourDto::from(hour))
    }

    pub async fn delete_hour(
        &self,
        business_id: Uuid,
        account_id: Uuid,
        day: String,
    ) -> Result<(), ServiceError> {
        let owner_id = self.get_owner_id(account_id).await?;

        let business = self
            .repo
            .find_by_id(business_id)
            .await?
            .ok_or(ServiceError::NotFound(ErrorDto {
                message: "business not found".to_owned(),
            }))?;

        if business.owner_id != owner_id {
            return Err(ServiceError::Forbidden(ErrorDto {
                message: "you do not own this business".to_owned(),
            }));
        }

        self.repo.delete_hour(business_id, day).await?;

        Ok(())
    }

    pub async fn get_media(
        &self,
        business_id: Uuid,
    ) -> Result<Vec<BusinessMediaDto>, ServiceError> {
        let media = self.repo.find_media(business_id).await?;

        Ok(media.into_iter().map(BusinessMediaDto::from).collect())
    }

    pub async fn create_media(
        &self,
        business_id: Uuid,
        account_id: Uuid,
        body: CreateBusinessMediaDto,
    ) -> Result<BusinessMediaDto, ServiceError> {
        let owner_id = self.get_owner_id(account_id).await?;

        let business = self
            .repo
            .find_by_id(business_id)
            .await?
            .ok_or(ServiceError::NotFound(ErrorDto {
                message: "business not found".to_owned(),
            }))?;

        if business.owner_id != owner_id {
            return Err(ServiceError::Forbidden(ErrorDto {
                message: "you do not own this business".to_owned(),
            }));
        }

        let media = self
            .repo
            .create_media(business_id, body.media_type, body.url)
            .await?;

        Ok(BusinessMediaDto::from(media))
    }

    pub async fn delete_media(
        &self,
        business_id: Uuid,
        account_id: Uuid,
        media_id: Uuid,
    ) -> Result<(), ServiceError> {
        let owner_id = self.get_owner_id(account_id).await?;

        let business = self
            .repo
            .find_by_id(business_id)
            .await?
            .ok_or(ServiceError::NotFound(ErrorDto {
                message: "business not found".to_owned(),
            }))?;

        if business.owner_id != owner_id {
            return Err(ServiceError::Forbidden(ErrorDto {
                message: "you do not own this business".to_owned(),
            }));
        }

        self.repo.delete_media(media_id, business_id).await?;

        Ok(())
    }
}
