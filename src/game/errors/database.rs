use thiserror::Error;

#[derive(Error, Debug)]
pub enum GameDatabaseError {
    #[error("Database connection failed: {msg}")]
    ConnectionFailed { msg: String },
    #[error("Database migrations failed: {msg}")]
    MigrationsFailed { msg: String },
    #[error("No database connection specified")]
    MissingConnection,
}

impl GameDatabaseError {
    pub fn connection_failed(msg: &str) -> Self {
        Self::ConnectionFailed {
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

    pub fn is_connection_failed(&self) -> bool {
        matches!(self, Self::ConnectionFailed { .. })
    }

    pub fn is_migrations_failed(&self) -> bool {
        matches!(self, Self::MigrationsFailed { .. })
    }

    pub fn is_missing_connection(&self) -> bool {
        matches!(self, Self::MissingConnection)
    }
}
