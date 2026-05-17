use crate::errors::DbError;
use crate::modules::business::business_entity::{
    BusinessEntity, BusinessHourEntity, BusinessMediaEntity,
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

    pub async fn find_nearby(
        &self,
        _latitude: f64,
        _longitude: f64,
    ) -> Result<Vec<BusinessEntity>, DbError> {
        todo!()
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
        phone_number: Option<i64>,
        title: Option<String>,
        logo: Option<String>,
        description: Option<String>,
        is_closed: Option<bool>,
    ) -> Result<BusinessEntity, DbError> {
        let business = sqlx::query_as!(
            BusinessEntity,
            r#"UPDATE business.businesses SET
               phone_number = COALESCE($2, phone_number),
               title = COALESCE($3, title),
               logo = COALESCE($4, logo),
               description = COALESCE($5, description),
               is_closed = COALESCE($6, is_closed)
               WHERE id = $1
               RETURNING id, phone_number, is_closed, title, logo, description, created_at, owner_id"#,
            id,
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

    pub async fn delete(&self, id: Uuid) -> Result<(), DbError> {
        sqlx::query!("DELETE FROM business.businesses WHERE id = $1", id)
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
        day: String,
        hours_type: String,
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
        day: String,
        hours_type: Option<String>,
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

    pub async fn delete_hour(&self, business_id: Uuid, day: String) -> Result<(), DbError> {
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
