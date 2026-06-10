use crate::shared::errors::DbError;
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

#[derive(Debug, FromRow)]
pub struct BusinessAddressEntity {
    pub address_line1: String,
    pub address_line2: String,
    pub landmark: Option<String>,
    pub pincode: String,
    pub city: String,
    pub state: String,
    pub latitude: f64,
    pub longitude: f64,
    pub radius: f64,
    pub business_id: Uuid,
}

#[derive(Debug)]
pub struct BusinessAddressRepo {
    pg: PgPool,
}

impl BusinessAddressRepo {
    pub fn new(pg: PgPool) -> Self {
        Self { pg }
    }

    pub async fn find(&self, business_id: Uuid) -> Result<Option<BusinessAddressEntity>, DbError> {
        let address = sqlx::query_as!(
            BusinessAddressEntity,
            r#"SELECT address_line1, address_line2, landmark, pincode, city, state, radius,
                      ST_Y(location::geometry) AS "latitude!",
                      ST_X(location::geometry) AS "longitude!",
                      business_id
               FROM business_addresses WHERE business_id = $1"#,
            business_id
        )
        .fetch_optional(&self.pg)
        .await?;

        Ok(address)
    }

    pub async fn upsert(
        &self,
        business_id: Uuid,
        address_line1: String,
        address_line2: String,
        landmark: Option<String>,
        pincode: String,
        city: String,
        state: String,
        latitude: f64,
        longitude: f64,
        radius: f64,
    ) -> Result<BusinessAddressEntity, DbError> {
        let address = sqlx::query_as!(
            BusinessAddressEntity,
            r#"INSERT INTO business_addresses
                   (business_id, address_line1, address_line2, landmark, pincode, city, state, location, radius)
               VALUES ($1, $2, $3, $4, $5, $6, $7, ST_SetSRID(ST_MakePoint($8, $9), 4326), $10)
               ON CONFLICT (business_id) DO UPDATE SET
                   address_line1 = EXCLUDED.address_line1,
                   address_line2 = EXCLUDED.address_line2,
                   landmark      = EXCLUDED.landmark,
                   pincode       = EXCLUDED.pincode,
                   city          = EXCLUDED.city,
                   state         = EXCLUDED.state,
                   location      = EXCLUDED.location,
                   radius        = EXCLUDED.radius
               RETURNING address_line1, address_line2, landmark, pincode, city, state, radius,
                         ST_Y(location::geometry) AS "latitude!",
                         ST_X(location::geometry) AS "longitude!",
                         business_id"#,
            business_id,
            address_line1,
            address_line2,
            landmark,
            pincode,
            city,
            state,
            longitude,
            latitude,
            radius,
        )
        .fetch_one(&self.pg)
        .await?;

        Ok(address)
    }

    pub async fn update(
        &self,
        business_id: Uuid,
        address_line1: Option<String>,
        address_line2: Option<String>,
        landmark: Option<String>,
        pincode: Option<String>,
        city: Option<String>,
        state: Option<String>,
        latitude: Option<f64>,
        longitude: Option<f64>,
        radius: Option<f64>,
    ) -> Result<BusinessAddressEntity, DbError> {
        let address = sqlx::query_as!(
            BusinessAddressEntity,
            r#"UPDATE business_addresses SET
                   address_line1 = COALESCE($2, address_line1),
                   address_line2 = COALESCE($3, address_line2),
                   landmark      = COALESCE($4, landmark),
                   pincode       = COALESCE($5, pincode),
                   city          = COALESCE($6, city),
                   state         = COALESCE($7, state),
                   location      = CASE
                                       WHEN $8::float8 IS NOT NULL AND $9::float8 IS NOT NULL
                                       THEN ST_SetSRID(ST_MakePoint($9, $8), 4326)
                                       ELSE location
                                   END,
                   radius        = COALESCE($10, radius)
               WHERE business_id = $1
               RETURNING address_line1, address_line2, landmark, pincode, city, state, radius,
                         ST_Y(location::geometry) AS "latitude!",
                         ST_X(location::geometry) AS "longitude!",
                         business_id"#,
            business_id,
            address_line1,
            address_line2,
            landmark,
            pincode,
            city,
            state,
            latitude,
            longitude,
            radius,
        )
        .fetch_one(&self.pg)
        .await?;

        Ok(address)
    }

    pub async fn delete(&self, business_id: Uuid) -> Result<(), DbError> {
        sqlx::query!(
            "DELETE FROM business_addresses WHERE business_id = $1",
            business_id
        )
        .execute(&self.pg)
        .await?;

        Ok(())
    }
}