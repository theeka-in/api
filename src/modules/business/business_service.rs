use crate::modules::auth::AuthService;
use crate::modules::business::business_dto::{
    BusinessAddressDto, BusinessDto, BusinessHourDto, BusinessMediaDto, CreateBusinessDto,
    CreateBusinessHourDto, CreateBusinessMediaDto, UpdateBusinessDto, UpdateBusinessHourDto,
};
use crate::modules::business::{
    BusinessWithOwnerAndAddressDto, CreateBusinessAddressDto, UpdateBusinessAddressDto,
};
use crate::modules::database::{BusinessHourType, DatabaseService, DayOfWeekType};
use crate::modules::users::UsersService;
use crate::shared::errors::{ErrorDto, ServiceError};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug)]
pub struct BusinessService {
    db: Arc<DatabaseService>,
    users: Arc<UsersService>,
    auth: Arc<AuthService>,
}

impl BusinessService {
    pub fn new(
        db: Arc<DatabaseService>,
        users_service: Arc<UsersService>,
        auth_service: Arc<AuthService>,
    ) -> Arc<Self> {
        Arc::new(Self {
            db,
            users: users_service,
            auth: auth_service,
        })
    }

    pub async fn get_business_by_id(
        &self,
        business_id: Uuid,
    ) -> Result<BusinessWithOwnerAndAddressDto, ServiceError> {
        let business = self
            .db
            .business
            .find_by_id_with_address_and_owner(business_id)
            .await?
            .ok_or(ServiceError::NotFound(ErrorDto {
                message: "business not found".to_owned(),
            }))?;

        Ok(BusinessWithOwnerAndAddressDto::from(business))
    }

    pub async fn get_business_by_id_and_owner(
        &self,
        business_id: Uuid,
        owner_id: Uuid,
    ) -> Result<BusinessDto, ServiceError> {
        let business = self
            .db
            .business
            .find_by_id_and_owner(business_id, owner_id)
            .await?
            .ok_or(ServiceError::NotFound(ErrorDto {
                message: "business not found".to_owned(),
            }))?;

        Ok(BusinessDto::from(business))
    }

    pub async fn get_all_businesses_by_owner(
        &self,
        owner_id: Uuid,
    ) -> Result<Vec<BusinessDto>, ServiceError> {
        let businesses = self.db.business.find_all_by_owner(owner_id).await?;

        Ok(businesses.into_iter().map(BusinessDto::from).collect())
    }

    pub async fn create_business(
        &self,
        owner_id: Uuid,
        body: CreateBusinessDto,
    ) -> Result<BusinessDto, ServiceError> {
        let account = self.auth.get_account_by_user_id(owner_id).await?;
        let phone_number = body.phone_number.unwrap_or(account.phone);

        let business = self
            .db
            .business
            .create(
                owner_id,
                phone_number,
                body.title,
                body.logo,
                body.description,
            )
            .await?;

        Ok(BusinessDto::from(business))
    }

    pub async fn update_business(
        &self,
        business_id: Uuid,
        owner_id: Uuid,
        body: UpdateBusinessDto,
    ) -> Result<BusinessDto, ServiceError> {
        self.get_business_by_id_and_owner(business_id, owner_id)
            .await?;

        let updated = self
            .db
            .business
            .update(
                business_id,
                owner_id,
                body.phone_number,
                body.title,
                body.logo,
                body.description,
                body.is_closed,
            )
            .await?;

        Ok(BusinessDto::from(updated))
    }

    pub async fn delete_business(
        &self,
        business_id: Uuid,
        owner_id: Uuid,
    ) -> Result<(), ServiceError> {
        self.get_business_by_id_and_owner(business_id, owner_id)
            .await?;

        self.db.business.delete(business_id, owner_id).await?;

        Ok(())
    }

    pub async fn get_address(&self, business_id: Uuid) -> Result<BusinessAddressDto, ServiceError> {
        let address =
            self.db
                .business_address
                .find(business_id)
                .await?
                .ok_or(ServiceError::NotFound(ErrorDto {
                    message: "address not found".to_owned(),
                }))?;

        Ok(BusinessAddressDto::from(address))
    }

    pub async fn create_address(
        &self,
        business_id: Uuid,
        owner_id: Uuid,
        dto: CreateBusinessAddressDto,
    ) -> Result<BusinessAddressDto, ServiceError> {
        self.get_business_by_id_and_owner(business_id, owner_id)
            .await?;

        let address = self
            .db
            .business_address
            .upsert(
                business_id,
                dto.address_line1,
                dto.address_line2,
                dto.landmark,
                dto.pincode,
                dto.city,
                dto.state,
                dto.latitude,
                dto.longitude,
                dto.radius,
            )
            .await?;

        Ok(BusinessAddressDto::from(address))
    }

    pub async fn update_address(
        &self,
        business_id: Uuid,
        owner_id: Uuid,
        dto: UpdateBusinessAddressDto,
    ) -> Result<BusinessAddressDto, ServiceError> {
        self.get_business_by_id_and_owner(business_id, owner_id)
            .await?;

        let address = self
            .db
            .business_address
            .update(
                business_id,
                dto.address_line1,
                dto.address_line2,
                dto.landmark,
                dto.pincode,
                dto.city,
                dto.state,
                dto.latitude,
                dto.longitude,
                dto.radius,
            )
            .await?;

        Ok(BusinessAddressDto::from(address))
    }

    pub async fn delete_address(
        &self,
        business_id: Uuid,
        owner_id: Uuid,
    ) -> Result<(), ServiceError> {
        self.get_business_by_id_and_owner(business_id, owner_id)
            .await?;

        self.db.business_address.delete(business_id).await?;
        Ok(())
    }

    pub async fn get_hours(&self, business_id: Uuid) -> Result<Vec<BusinessHourDto>, ServiceError> {
        let hours = self.db.business_hour.find_all(business_id).await?;

        Ok(hours.into_iter().map(BusinessHourDto::from).collect())
    }

    pub async fn create_hour(
        &self,
        business_id: Uuid,
        owner_id: Uuid,
        body: CreateBusinessHourDto,
    ) -> Result<BusinessHourDto, ServiceError> {
        self.get_business_by_id_and_owner(business_id, owner_id)
            .await?;

        if matches!(body.hours_type, BusinessHourType::CustomRange) {
            if (matches!(body.open_time, None)) {
                return Err(ServiceError::Forbidden(ErrorDto {
                    message: "open_time is not defined".to_owned(),
                }));
            }

            if (matches!(body.close_time, None)) {
                return Err(ServiceError::Forbidden(ErrorDto {
                    message: "close_time is not defined".to_owned(),
                }));
            }
        } else {
            if (matches!(body.open_time, Some(_))) {
                return Err(ServiceError::Forbidden(ErrorDto {
                    message: "open_time is defined".to_owned(),
                }));
            }

            if (matches!(body.close_time, Some(_))) {
                return Err(ServiceError::Forbidden(ErrorDto {
                    message: "close_time is defined".to_owned(),
                }));
            }
        }

        let hour = self
            .db
            .business_hour
            .create(
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
        owner_id: Uuid,
        day: DayOfWeekType,
        body: UpdateBusinessHourDto,
    ) -> Result<BusinessHourDto, ServiceError> {
        self.get_business_by_id_and_owner(business_id, owner_id)
            .await?;

        dbg!("{}", &body);

        if let Some(hours_type) = &body.hours_type {
            if matches!(hours_type, BusinessHourType::CustomRange) {
                if (matches!(body.open_time, None)) {
                    return Err(ServiceError::Forbidden(ErrorDto {
                        message: "open_time is not defined".to_owned(),
                    }));
                }

                if (matches!(body.close_time, None)) {
                    return Err(ServiceError::Forbidden(ErrorDto {
                        message: "close_time is not defined".to_owned(),
                    }));
                }
            } else {
                if (matches!(body.open_time, Some(_))) {
                    return Err(ServiceError::Forbidden(ErrorDto {
                        message: "open_time is defined".to_owned(),
                    }));
                }

                if (matches!(body.close_time, Some(_))) {
                    return Err(ServiceError::Forbidden(ErrorDto {
                        message: "close_time is defined".to_owned(),
                    }));
                }
            }
        }

        let hour = self
            .db
            .business_hour
            .update(
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
        owner_id: Uuid,
        day: DayOfWeekType,
    ) -> Result<(), ServiceError> {
        self.get_business_by_id_and_owner(business_id, owner_id)
            .await?;

        self.db.business_hour.delete(business_id, day).await?;

        Ok(())
    }

    pub async fn get_media(
        &self,
        business_id: Uuid,
    ) -> Result<Vec<BusinessMediaDto>, ServiceError> {
        let media = self.db.business_media.find_all(business_id).await?;

        Ok(media.into_iter().map(BusinessMediaDto::from).collect())
    }

    pub async fn create_media(
        &self,
        business_id: Uuid,
        owner_id: Uuid,
        body: CreateBusinessMediaDto,
    ) -> Result<BusinessMediaDto, ServiceError> {
        self.get_business_by_id_and_owner(business_id, owner_id)
            .await?;

        let media = self
            .db
            .business_media
            .create(business_id, body.media_type, body.url)
            .await?;

        Ok(BusinessMediaDto::from(media))
    }

    pub async fn delete_media(
        &self,
        business_id: Uuid,
        owner_id: Uuid,
        media_id: Uuid,
    ) -> Result<(), ServiceError> {
        self.get_business_by_id_and_owner(business_id, owner_id)
            .await?;

        self.db.business_media.delete(media_id, business_id).await?;

        Ok(())
    }
}
