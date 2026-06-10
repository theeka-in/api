use crate::shared::errors::DbError;
use poem_openapi::Enum;
use sqlx::{FromRow, PgPool, types::chrono};
use uuid::Uuid;

#[derive(Debug, sqlx::Type, PartialEq, Enum)]
#[sqlx(type_name = "day_of_week", rename_all = "snake_case")]
pub enum DayOfWeekType {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

#[derive(Debug, sqlx::Type, PartialEq, Enum)]
#[sqlx(type_name = "business_hour_type", rename_all = "snake_case")]
pub enum BusinessHourType {
    Closed,
    #[sqlx(rename = "open_24_hours")]
    Open24Hours,
    CustomRange,
}

#[derive(Debug, FromRow)]
pub struct BusinessHourEntity {
    pub id: Uuid,
    pub day: DayOfWeekType,
    pub hours_type: BusinessHourType,
    pub open_time: Option<chrono::NaiveTime>,
    pub close_time: Option<chrono::NaiveTime>,
    pub business_id: Uuid,
}

#[derive(Debug)]
pub struct BusinessHourRepo {
    pg: PgPool,
}

impl BusinessHourRepo {
    pub fn new(pg: PgPool) -> Self {
        Self { pg }
    }

    pub async fn find_all(&self, business_id: Uuid) -> Result<Vec<BusinessHourEntity>, DbError> {
        let hours = sqlx::query_as!(
            BusinessHourEntity,
            r#"SELECT id, day AS "day: _", hours_type AS "hours_type: _", open_time, close_time, business_id
               FROM business_hours WHERE business_id = $1"#,
            business_id
        )
        .fetch_all(&self.pg)
        .await?;

        Ok(hours)
    }

    pub async fn create(
        &self,
        business_id: Uuid,
        day: DayOfWeekType,
        hours_type: BusinessHourType,
        open_time: Option<String>,
        close_time: Option<String>,
    ) -> Result<BusinessHourEntity, DbError> {
        let open_time = open_time
            .and_then(|t| chrono::NaiveTime::parse_from_str(&t, "%H:%M").ok());

        let close_time = close_time
            .and_then(|t| chrono::NaiveTime::parse_from_str(&t, "%H:%M").ok());

        let hour = sqlx::query_as!(
            BusinessHourEntity,
            r#"INSERT INTO business_hours (id, business_id, day, hours_type, open_time, close_time)
               VALUES (gen_random_uuid(), $1, $2::day_of_week, $3::business_hour_type, $4, $5)
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

    pub async fn update(
        &self,
        business_id: Uuid,
        day: DayOfWeekType,
        hours_type: Option<BusinessHourType>,
        open_time: Option<String>,
        close_time: Option<String>,
    ) -> Result<BusinessHourEntity, DbError> {
        let open_time = open_time
            .and_then(|t| chrono::NaiveTime::parse_from_str(&t, "%H:%M").ok());

        let close_time = close_time
            .and_then(|t| chrono::NaiveTime::parse_from_str(&t, "%H:%M").ok());

        let hour = sqlx::query_as!(
            BusinessHourEntity,
            r#"UPDATE business_hours SET
               hours_type = COALESCE($3::business_hour_type, hours_type),
               open_time  = COALESCE($4, open_time),
               close_time = COALESCE($5, close_time)
               WHERE business_id = $1 AND day = $2::day_of_week
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

    pub async fn delete(&self, business_id: Uuid, day: DayOfWeekType) -> Result<(), DbError> {
        sqlx::query!(
            "DELETE FROM business_hours WHERE business_id = $1 AND day = $2::day_of_week",
            business_id,
            day as _
        )
        .execute(&self.pg)
        .await?;

        Ok(())
    }
}