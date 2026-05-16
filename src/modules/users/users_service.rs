use crate::errors::{DbError, ErrorDto, ServiceError};
use crate::modules::users::users_dto::{
    CreateUserAddressDto, CreateUserDto, UpdateUserAddressDto, UpdateUserDto, UserAddressDto, UserDto
};
use crate::modules::users::users_repository::UsersRepository;
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
        todo!()
    }

    pub async fn create(
        &self,
        account_id: Uuid,
        body: CreateUserDto,
    ) -> Result<UserDto, ServiceError> {
        todo!()
    }

    pub async fn update(
        &self,
        account_id: Uuid,
        body: UpdateUserDto,
    ) -> Result<UserDto, ServiceError> {
        todo!()
    }

    pub async fn delete(&self, account_id: Uuid) -> Result<(), ServiceError> {
        todo!()
    }

    pub async fn get_addresses(&self, user_id: Uuid) -> Result<Vec<UserAddressDto>, ServiceError> {
        todo!()
    }

    pub async fn create_address(
        &self,
        user_id: Uuid,
        body: CreateUserAddressDto,
    ) -> Result<UserAddressDto, ServiceError> {
        todo!()
    }

    pub async fn update_address(
        &self,
        user_id: Uuid,
        address_id: Uuid,
        body: UpdateUserAddressDto,
    ) -> Result<UserAddressDto, ServiceError> {
        todo!()
    }

    pub async fn delete_address(
        &self,
        user_id: Uuid,
        address_id: Uuid,
    ) -> Result<(), ServiceError> {
        todo!()
    }
}
