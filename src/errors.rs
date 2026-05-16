use poem_openapi::Object;
use serde::Serialize;
use sqlx::error::DatabaseError;

#[derive(Debug, Object, Serialize)]
pub struct ErrorDto {
    pub message: String,
}

#[derive(Debug)]
pub enum ServiceError {
    NotFound(ErrorDto),

    Conflict(ErrorDto),

    Unauthorized(ErrorDto),

    Forbidden(ErrorDto),

    Internal(ErrorDto),
}

impl From<DbError> for ServiceError {
    fn from(e: DbError) -> Self {
        match e {
            DbError::NotFound => ServiceError::NotFound(ErrorDto {
                message: "row not found".to_owned(),
            }),
            DbError::UniqueViolation { constraint } => ServiceError::Conflict(ErrorDto {
                message: format!("{constraint} already exists"),
            }),
            DbError::ForeignKeyViolation { constraint } => ServiceError::NotFound(ErrorDto {
                message: format!("{constraint} not found"),
            }),
            DbError::CheckViolation { constraint } => ServiceError::Conflict(ErrorDto {
                message: format!("{constraint} is invalid"),
            }),
            DbError::PoolTimedOut => ServiceError::Internal(ErrorDto {
                message: "database pool timed out".to_owned(),
            }),
            DbError::PoolClosed => ServiceError::Internal(ErrorDto {
                message: "database pool closed".to_owned(),
            }),
            DbError::ColumnNotFound(col) => ServiceError::Internal(ErrorDto {
                message: format!("column {col} not found"),
            }),
            DbError::Decode(msg) => ServiceError::Internal(ErrorDto {
                message: format!("decode error: {msg}"),
            }),
            DbError::Internal(msg) => ServiceError::Internal(ErrorDto { message: msg }),
        }
    }
}

#[derive(Debug)]
pub enum DbError {
    NotFound,

    UniqueViolation { constraint: String },
    ForeignKeyViolation { constraint: String },
    CheckViolation { constraint: String },

    PoolTimedOut,
    PoolClosed,

    ColumnNotFound(String),
    Decode(String),

    Internal(String),
}

fn get_constraint_from_error(e: Box<dyn DatabaseError>) -> String {
    e.constraint()
        .unwrap_or("unknown")
        .replace("_key", "")
        .split("_")
        .skip(1)
        .collect::<Vec<&str>>()
        .join(" ")
}

impl From<sqlx::Error> for DbError {
    fn from(e: sqlx::Error) -> Self {
        match e {
            sqlx::Error::RowNotFound => DbError::NotFound,

            sqlx::Error::Database(e) if e.is_unique_violation() => DbError::UniqueViolation {
                constraint: get_constraint_from_error(e),
            },

            sqlx::Error::Database(e) if e.is_foreign_key_violation() => {
                DbError::ForeignKeyViolation {
                    constraint: get_constraint_from_error(e),
                }
            }
            sqlx::Error::Database(e) if e.is_check_violation() => DbError::CheckViolation {
                constraint: get_constraint_from_error(e),
            },
            sqlx::Error::Database(e) => DbError::Internal(e.to_string()),

            sqlx::Error::PoolTimedOut => DbError::PoolTimedOut,
            sqlx::Error::PoolClosed => DbError::PoolClosed,

            sqlx::Error::ColumnNotFound(col) => DbError::ColumnNotFound(col),

            sqlx::Error::Decode(e) => DbError::Decode(e.to_string()),

            e => DbError::Internal(e.to_string()),
        }
    }
}
