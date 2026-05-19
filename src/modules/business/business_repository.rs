use crate::errors::DbError;
use crate::modules::business::business_entity::{
    BusinessAddressEntity, BusinessEntity, BusinessHourEntity, BusinessHourType,
    BusinessMediaEntity, DayOfWeekType,
};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug)]
pub struct BusinessRepository {
    pg: PgPool,
}

impl BusinessRepository {
    pub fn new(pg: PgPool) -> Self {
        Self { pg }
    }

    pub async fn find_business_by_id(&self, id: Uuid) -> Result<Option<BusinessEntity>, DbError> {
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

    pub async fn find_business_by_id_and_owner(
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

    pub async fn find_all_businesses_by_owner(
        &self,
        owner_id: Uuid,
    ) -> Result<Vec<BusinessEntity>, DbError> {
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

    pub async fn create_business(
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

    pub async fn update_business(
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
               title = COALESCE($4, title),
               logo = COALESCE($5, logo),
               description = COALESCE($6, description),
               is_closed = COALESCE($7, is_closed)
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

    pub async fn delete_business(&self, id: Uuid, owner_id: Uuid) -> Result<(), DbError> {
        sqlx::query!(
            "DELETE FROM business.businesses WHERE id = $1 AND owner_id = $2",
            id,
            owner_id
        )
        .execute(&self.pg)
        .await?;

        Ok(())
    }

    pub async fn find_address(
        &self,
        business_id: Uuid,
    ) -> Result<Option<BusinessAddressEntity>, DbError> {
        let address = sqlx::query_as!(
            BusinessAddressEntity,
            r#"SELECT complete_address, city, state, pincode, latitude, longitude, radius, business_id
               FROM business.business_addresses WHERE business_id = $1"#,
            business_id
        )
        .fetch_optional(&self.pg)
        .await?;

        Ok(address)
    }

    pub async fn create_address(
        &self,
        business_id: Uuid,
        complete_address: String,
        city: String,
        state: String,
        pincode: i32,
        latitude: f64,
        longitude: f64,
        radius: f64,
    ) -> Result<BusinessAddressEntity, DbError> {
        let address = sqlx::query_as!(
            BusinessAddressEntity,
            r#"INSERT INTO business.business_addresses
                   (business_id, complete_address, city, state, pincode, latitude, longitude, radius)
               VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
               ON CONFLICT (business_id) DO UPDATE SET
                   complete_address = EXCLUDED.complete_address,
                   city             = EXCLUDED.city,
                   state            = EXCLUDED.state,
                   pincode          = EXCLUDED.pincode,
                   latitude         = EXCLUDED.latitude,
                   longitude        = EXCLUDED.longitude,
                   radius           = EXCLUDED.radius
               RETURNING complete_address, city, state, pincode, latitude, longitude, radius, business_id"#,
            business_id,
            complete_address,
            city,
            state,
            pincode,
            latitude,
            longitude,
            radius
        )
        .fetch_one(&self.pg)
        .await?;

        Ok(address)
    }

    pub async fn update_address(
        &self,
        business_id: Uuid,
        complete_address: Option<String>,
        city: Option<String>,
        state: Option<String>,
        pincode: Option<i32>,
        latitude: Option<f64>,
        longitude: Option<f64>,
        radius: Option<f64>,
    ) -> Result<BusinessAddressEntity, DbError> {
        let address = sqlx::query_as!(
            BusinessAddressEntity,
            r#"UPDATE business.business_addresses SET
                   complete_address = COALESCE($2, complete_address),
                   city             = COALESCE($3, city),
                   state            = COALESCE($4, state),
                   pincode          = COALESCE($5, pincode),
                   latitude         = COALESCE($6, latitude),
                   longitude        = COALESCE($7, longitude),
                   radius           = COALESCE($8, radius)
               WHERE business_id = $1
               RETURNING complete_address, city, state, pincode, latitude, longitude, radius, business_id"#,
            business_id,
            complete_address,
            city,
            state,
            pincode,
            latitude,
            longitude,
            radius
        )
        .fetch_one(&self.pg)
        .await?;
    
        Ok(address)
    }

    pub async fn delete_address(&self, business_id: Uuid) -> Result<(), DbError> {
        sqlx::query!(
            "DELETE FROM business.business_addresses WHERE business_id = $1",
            business_id
        )
        .execute(&self.pg)
        .await?;

        Ok(())
    }

    pub async fn find_hours(&self, business_id: Uuid) -> Result<Vec<BusinessHourEntity>, DbError> {
        let hours = sqlx::query_as!(
            BusinessHourEntity,
            r#"SELECT id, day AS "day: _", hours_type AS "hours_type: _", open_time, close_time, business_id
               FROM business.business_hours WHERE business_id = $1"#,
            business_id
        )
        .fetch_all(&self.pg)
        .await?;

        Ok(hours)
    }

    pub async fn create_hour(
        &self,
        business_id: Uuid,
        day: DayOfWeekType,
        hours_type: BusinessHourType,
        open_time: Option<String>,
        close_time: Option<String>,
    ) -> Result<BusinessHourEntity, DbError> {
        let open_time = open_time
            .map(|t| sqlx::types::chrono::NaiveTime::parse_from_str(&t, "%H:%M").ok())
            .flatten();

        let close_time = close_time
            .map(|t| sqlx::types::chrono::NaiveTime::parse_from_str(&t, "%H:%M").ok())
            .flatten();

        let hour = sqlx::query_as!(
            BusinessHourEntity,
            r#"INSERT INTO business.business_hours (id, business_id, day, hours_type, open_time, close_time)
               VALUES (gen_random_uuid(), $1, $2::business.day_of_week, $3::business.business_hour_type, $4, $5)
               RETURNING id, day AS "day: _", hours_type AS "hours_type: _", open_time, close_time, business_id"#,
            business_id,
            day as _,
            hours_type as _,
            open_time,
            close_time
        )
        .fetch_one(&self.pg)
        .await?;

        Ok(hour)
    }

    pub async fn update_hour(
        &self,
        business_id: Uuid,
        day: DayOfWeekType,
        hours_type: Option<BusinessHourType>,
        open_time: Option<String>,
        close_time: Option<String>,
    ) -> Result<BusinessHourEntity, DbError> {
        let open_time = open_time
            .map(|t| sqlx::types::chrono::NaiveTime::parse_from_str(&t, "%H:%M").ok())
            .flatten();

        let close_time = close_time
            .map(|t| sqlx::types::chrono::NaiveTime::parse_from_str(&t, "%H:%M").ok())
            .flatten();

        let hour = sqlx::query_as!(
            BusinessHourEntity,
            r#"UPDATE business.business_hours SET
               hours_type = COALESCE($3::business.business_hour_type, hours_type),
               open_time = COALESCE($4, open_time),
               close_time = COALESCE($5, close_time)
               WHERE business_id = $1 AND day = $2::business.day_of_week
               RETURNING id, day AS "day: _", hours_type AS "hours_type: _", open_time, close_time, business_id"#,
            business_id,
            day as _,
            hours_type as _,
            open_time,
            close_time
        )
        .fetch_one(&self.pg)
        .await?;

        Ok(hour)
    }

    pub async fn delete_hour(&self, business_id: Uuid, day: DayOfWeekType) -> Result<(), DbError> {
        sqlx::query!(
            "DELETE FROM business.business_hours WHERE business_id = $1 AND day = $2::business.day_of_week",
            business_id,
            day as _
        )
        .execute(&self.pg)
        .await?;

        Ok(())
    }

    pub async fn find_media(&self, business_id: Uuid) -> Result<Vec<BusinessMediaEntity>, DbError> {
        let media = sqlx::query_as!(
            BusinessMediaEntity,
            r#"SELECT id, type AS "media_type: _", url, business_id
               FROM business.business_media WHERE business_id = $1"#,
            business_id
        )
        .fetch_all(&self.pg)
        .await?;

        Ok(media)
    }

    pub async fn create_media(
        &self,
        business_id: Uuid,
        media_type: String,
        url: String,
    ) -> Result<BusinessMediaEntity, DbError> {
        let media = sqlx::query_as!(
            BusinessMediaEntity,
            r#"INSERT INTO business.business_media (id, business_id, type, url)
               VALUES (gen_random_uuid(), $1, $2::shared.media_type, $3)
               RETURNING id, type AS "media_type: _", url, business_id"#,
            business_id,
            media_type as _,
            url
        )
        .fetch_one(&self.pg)
        .await?;

        Ok(media)
    }

    pub async fn delete_media(&self, id: Uuid, business_id: Uuid) -> Result<(), DbError> {
        sqlx::query!(
            "DELETE FROM business.business_media WHERE id = $1 AND business_id = $2",
            id,
            business_id
        )
        .execute(&self.pg)
        .await?;

        Ok(())
    }
}
