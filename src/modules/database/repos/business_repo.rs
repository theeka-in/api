use crate::{
    modules::database::{BusinessAddressEntity, UserEntity},
    shared::errors::DbError,
};
use sqlx::{FromRow, PgPool, types::chrono};
use uuid::Uuid;

#[derive(Debug, FromRow)]
pub struct BusinessEntity {
    pub id: Uuid,
    pub phone_number: i64,
    pub is_closed: bool,
    pub title: String,
    pub logo: Option<String>,
    pub description: Option<String>,
    pub created_at: chrono::NaiveDateTime,
    pub owner_id: Uuid,
}

#[derive(Debug)]
pub struct BusinessRepo {
    pg: PgPool,
}

impl BusinessRepo {
    pub fn new(pg: PgPool) -> Self {
        Self { pg }
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<BusinessEntity>, DbError> {
        let business = sqlx::query_as!(
            BusinessEntity,
            r#"SELECT id, phone_number, is_closed, title, logo, description, created_at, owner_id
               FROM business.businesses WHERE id = $1"#,
            id
        )
        .fetch_optional(&self.pg)
        .await?;

        Ok(business)
    }

    pub async fn find_by_id_with_address_and_owner(
        &self,
        id: Uuid,
    ) -> Result<Option<(BusinessEntity, BusinessAddressEntity, UserEntity)>, DbError> {
        let row = sqlx::query!(
            r#"SELECT
                b.id, 
                b.phone_number, 
                b.is_closed, 
                b.title, 
                b.logo, 
                b.description, 
                b.created_at, 
                b.owner_id,
                a.address_line1, 
                a.address_line2, 
                a.landmark, 
                a.city, 
                a.state, 
                a.pincode, 
                a.radius,
                ST_X(a.location::geometry) AS longitude,
                ST_Y(a.location::geometry) AS latitude,
                u.name AS owner_name, 
                u.avatar AS owner_avatar, 
                u.account_id AS owner_account_id
               FROM business.businesses b
               INNER JOIN business.business_addresses a ON a.business_id = b.id
               INNER JOIN users.users u ON u.id = b.owner_id
               WHERE b.id = $1"#,
            id
        )
        .fetch_optional(&self.pg)
        .await?;

        Ok(row.map(|row| {
            let business = BusinessEntity {
                id: row.id,
                phone_number: row.phone_number,
                is_closed: row.is_closed,
                title: row.title,
                logo: row.logo,
                description: row.description,
                created_at: row.created_at,
                owner_id: row.owner_id,
            };
            let address = BusinessAddressEntity {
                address_line1: row.address_line1,
                address_line2: row.address_line2,
                landmark: row.landmark,
                city: row.city,
                state: row.state,
                pincode: row.pincode,
                latitude: row.latitude.unwrap_or(0.0),
                longitude: row.longitude.unwrap_or(0.0),
                radius: row.radius,
                business_id: row.id,
            };
            let owner = UserEntity {
                id: row.owner_id,
                name: row.owner_name,
                avatar: row.owner_avatar,
                account_id: row.owner_account_id,
            };
            (business, address, owner)
        }))
    }

    pub async fn find_by_id_and_owner(
        &self,
        id: Uuid,
        owner_id: Uuid,
    ) -> Result<Option<BusinessEntity>, DbError> {
        let business = sqlx::query_as!(
            BusinessEntity,
            r#"SELECT id, phone_number, is_closed, title, logo, description, created_at, owner_id
               FROM business.businesses WHERE id = $1 AND owner_id = $2"#,
            id,
            owner_id
        )
        .fetch_optional(&self.pg)
        .await?;

        Ok(business)
    }

    pub async fn find_all_by_owner(&self, owner_id: Uuid) -> Result<Vec<BusinessEntity>, DbError> {
        let businesses = sqlx::query_as!(
            BusinessEntity,
            r#"SELECT id, phone_number, is_closed, title, logo, description, created_at, owner_id
               FROM business.businesses WHERE owner_id = $1"#,
            owner_id
        )
        .fetch_all(&self.pg)
        .await?;

        Ok(businesses)
    }

    pub async fn create(
        &self,
        owner_id: Uuid,
        phone_number: i64,
        title: String,
        logo: Option<String>,
        description: Option<String>,
    ) -> Result<BusinessEntity, DbError> {
        let business = sqlx::query_as!(
            BusinessEntity,
            r#"INSERT INTO business.businesses (id, owner_id, phone_number, title, logo, description)
               VALUES (gen_random_uuid(), $1, $2, $3, $4, $5)
               RETURNING id, phone_number, is_closed, title, logo, description, created_at, owner_id"#,
            owner_id,
            phone_number,
            title,
            logo,
            description
        )
        .fetch_one(&self.pg)
        .await?;

        Ok(business)
    }

    pub async fn update(
        &self,
        id: Uuid,
        owner_id: Uuid,
        phone_number: Option<i64>,
        title: Option<String>,
        logo: Option<String>,
        description: Option<String>,
        is_closed: Option<bool>,
    ) -> Result<BusinessEntity, DbError> {
        let business = sqlx::query_as!(
            BusinessEntity,
            r#"UPDATE business.businesses SET
               phone_number = COALESCE($3, phone_number),
               title        = COALESCE($4, title),
               logo         = COALESCE($5, logo),
               description  = COALESCE($6, description),
               is_closed    = COALESCE($7, is_closed)
               WHERE id = $1 AND owner_id = $2
               RETURNING id, phone_number, is_closed, title, logo, description, created_at, owner_id"#,
            id,
            owner_id,
            phone_number,
            title,
            logo,
            description,
            is_closed
        )
        .fetch_one(&self.pg)
        .await?;

        Ok(business)
    }

    pub async fn delete(&self, id: Uuid, owner_id: Uuid) -> Result<(), DbError> {
        sqlx::query!(
            "DELETE FROM business.businesses WHERE id = $1 AND owner_id = $2",
            id,
            owner_id
        )
        .execute(&self.pg)
        .await?;

        Ok(())
    }
}
