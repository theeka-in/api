use crate::modules::database::DatabaseService;
use crate::modules::users::users_dto::{
    CreateUserAddressDto, CreateUserDto, UpdateUserAddressDto, UpdateUserDto, UserAddressDto,
    UserDto,
};
use crate::shared::errors::{DbError, ErrorDto, ServiceError};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug)]
pub struct UsersService {
    db: Arc<DatabaseService>,
}

impl UsersService {
    pub fn new(db: Arc<DatabaseService>) -> Arc<Self> {
        Arc::new(Self { db })
    }

    pub async fn get(&self, account_id: Uuid) -> Result<UserDto, ServiceError> {
        let user =
            self.db
                .user
                .find_by_account(account_id)
                .await?
                .ok_or(ServiceError::NotFound(ErrorDto {
                    message: "user not found".to_owned(),
                }))?;

        Ok(user.into())
    }

    pub async fn create(
        &self,
        account_id: Uuid,
        body: CreateUserDto,
    ) -> Result<UserDto, ServiceError> {
        if self.db.user.find_by_account(account_id).await?.is_some() {
            return Err(ServiceError::Conflict(ErrorDto {
                message: "user already exists".to_owned(),
            }));
        }

        let user = self
            .db
            .user
            .create(account_id, body.name, body.avatar)
            .await?;

        Ok(user.into())
    }

    pub async fn update(
        &self,
        account_id: Uuid,
        body: UpdateUserDto,
    ) -> Result<UserDto, ServiceError> {
        let existing = self.get(account_id).await?;

        let user = self
            .db
            .user
            .update(existing.id, body.name, body.avatar)
            .await?;

        Ok(user.into())
    }

    pub async fn get_addresses(
        &self,
        account_id: Uuid,
    ) -> Result<Vec<UserAddressDto>, ServiceError> {
        let user = self.get(account_id).await?;

        let addresses = self.db.user_address.find_all(user.id).await?;

        Ok(addresses.into_iter().map(UserAddressDto::from).collect())
    }

    pub async fn create_address(
        &self,
        account_id: Uuid,
        body: CreateUserAddressDto,
    ) -> Result<UserAddressDto, ServiceError> {
        let user = self.get(account_id).await?;

        let address = self
            .db
            .user_address
            .create(
                user.id,
                body.name,
                body.complete_address,
                body.city,
                body.state,
                body.pincode,
                body.latitude,
                body.longitude,
            )
            .await?;

        Ok(address.into())
    }

    pub async fn update_address(
        &self,
        account_id: Uuid,
        address_id: Uuid,
        body: UpdateUserAddressDto,
    ) -> Result<UserAddressDto, ServiceError> {
        let user = self.get(account_id).await?;

        let address = self
            .db
            .user_address
            .update(
                address_id,
                user.id,
                body.name,
                body.complete_address,
                body.city,
                body.state,
                body.pincode,
                body.latitude,
                body.longitude,
            )
            .await?;

        Ok(address.into())
    }

    pub async fn delete_address(
        &self,
        account_id: Uuid,
        address_id: Uuid,
    ) -> Result<(), ServiceError> {
        let user = self.get(account_id).await?;

        self.db.user_address.delete(address_id, user.id).await?;

        Ok(())
    }
}
