use crate::shared::errors::{DbError, ErrorDto, ServiceError};
use crate::features::users::users_dto::{
    CreateUserAddressDto, CreateUserDto, UpdateUserAddressDto, UpdateUserDto, UserAddressDto,
    UserDto,
};
use crate::features::users::users_repository::UsersRepository;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug)]
pub struct UsersService {
    repo: UsersRepository,
}

impl UsersService {
    pub fn new(repo: UsersRepository) -> Arc<Self> {
        Arc::new(Self { repo })
    }

    pub async fn get(&self, account_id: Uuid) -> Result<UserDto, ServiceError> {
        let user =
            self.repo
                .find_by_account_id(account_id)
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
        if self.repo.find_by_account_id(account_id).await?.is_some() {
            return Err(ServiceError::Conflict(ErrorDto {
                message: "user already exists".to_owned(),
            }));
        }

        let user = self.repo.create(account_id, body.name, body.avatar).await?;

        Ok(user.into())
    }

    pub async fn update(
        &self,
        account_id: Uuid,
        body: UpdateUserDto,
    ) -> Result<UserDto, ServiceError> {
        let existing = self.get(account_id).await?;

        let user = self
            .repo
            .update(existing.id, body.name, body.avatar)
            .await?;

        Ok(user.into())
    }

    pub async fn delete(&self, account_id: Uuid) -> Result<(), ServiceError> {
        let existing = self.get(account_id).await?;

        self.repo.delete(existing.id).await?;

        Ok(())
    }

    pub async fn get_addresses(
        &self,
        account_id: Uuid,
    ) -> Result<Vec<UserAddressDto>, ServiceError> {
        let user = self.get(account_id).await?;

        let addresses = self.repo.find_addresses(user.id).await?;

        Ok(addresses.into_iter().map(UserAddressDto::from).collect())
    }

    pub async fn create_address(
        &self,
        account_id: Uuid,
        body: CreateUserAddressDto,
    ) -> Result<UserAddressDto, ServiceError> {
        let user = self.get(account_id).await?;

        let address = self
            .repo
            .create_address(
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
            .repo
            .update_address(
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

        self.repo.delete_address(address_id, user.id).await?;

        Ok(())
    }
}
