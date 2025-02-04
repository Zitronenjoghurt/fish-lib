use thiserror::Error;

#[derive(Error, Debug)]
pub enum GameDatabaseError {
    #[error("Database connection failed: {msg}")]
    ConnectionFailed { msg: String },
    #[error("Database foreign key violation: {msg}")]
    ForeignKeyViolation { msg: String },
    #[error("Database migrations failed: {msg}")]
    MigrationsFailed { msg: String },
    #[error("No database connection specified")]
    MissingConnection,
    #[error("Record not found")]
    NotFound,
    #[error("Database error: {msg}")]
    Other { msg: String },
    #[error("Database unique constraint violation: {msg}")]
    UniqueConstraintViolation { msg: String },
}

impl GameDatabaseError {
    pub fn connection_failed(msg: &str) -> Self {
        Self::ConnectionFailed {
            msg: msg.to_string(),
        }
    }

    pub fn foreign_key_violation(msg: &str) -> Self {
        Self::ForeignKeyViolation {
            msg: msg.to_string(),
        }
    }

    pub fn migrations_failed(msg: &str) -> Self {
        Self::MigrationsFailed {
            msg: msg.to_string(),
        }
    }

    pub fn missing_connection() -> Self {
        Self::MissingConnection
    }

    pub fn not_found() -> Self {
        Self::NotFound
    }

    pub fn other(msg: &str) -> Self {
        Self::Other {
            msg: msg.to_string(),
        }
    }

    pub fn unique_constraint_violation(msg: &str) -> Self {
        Self::UniqueConstraintViolation {
            msg: msg.to_string(),
        }
    }

    pub fn is_connection_failed(&self) -> bool {
        matches!(self, Self::ConnectionFailed { .. })
    }

    pub fn is_foreign_key_violation(&self) -> bool {
        matches!(self, Self::ForeignKeyViolation { .. })
    }

    pub fn is_migrations_failed(&self) -> bool {
        matches!(self, Self::MigrationsFailed { .. })
    }

    pub fn is_missing_connection(&self) -> bool {
        matches!(self, Self::MissingConnection)
    }

    pub fn is_not_found(&self) -> bool {
        matches!(self, Self::NotFound)
    }

    pub fn is_other(&self) -> bool {
        matches!(self, Self::Other { .. })
    }

    pub fn is_unique_constraint_violation(&self) -> bool {
        matches!(self, Self::UniqueConstraintViolation { .. })
    }
}
