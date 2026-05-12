use sqlx::error::DatabaseError;
use std::fmt;

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

impl fmt::Display for DbError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DbError::NotFound => write!(f, "record not found"),

            DbError::UniqueViolation { constraint } => {
                write!(f, "unique constraint violation on: {}", constraint)
            }
            DbError::ForeignKeyViolation { constraint } => {
                write!(f, "foreign key violation on: {}", constraint)
            }
            DbError::CheckViolation { constraint } => {
                write!(f, "check constraint violation on: {}", constraint)
            }

            DbError::PoolTimedOut => write!(f, "database pool timed out"),
            DbError::PoolClosed => write!(f, "database pool closed"),

            DbError::ColumnNotFound(col) => write!(f, "column not found: {}", col),
            DbError::Decode(e) => write!(f, "type decode error: {}", e),

            DbError::Internal(msg) => write!(f, "database error: {}", msg),
        }
    }
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
