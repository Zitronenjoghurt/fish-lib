use crate::game::errors::database::GameDatabaseError;
use diesel::result::DatabaseErrorKind;
use std::error::Error;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum GameRepositoryError {
    #[error("Database error: {0}")]
    Database(#[from] GameDatabaseError),
    #[error("Unexpected error: {msg}")]
    Unexpected {
        msg: String,
        #[source]
        source: Box<dyn std::error::Error>,
    },
}

impl GameRepositoryError {
    pub fn database(error: GameDatabaseError) -> Self {
        GameRepositoryError::Database(error)
    }

    pub fn unexpected(error: Box<dyn std::error::Error>) -> Self {
        Self::Unexpected {
            msg: error.to_string(),
            source: error,
        }
    }

    pub fn is_database_error(&self) -> bool {
        matches!(self, GameRepositoryError::Database(_))
    }

    pub fn get_database_error(&self) -> Option<&GameDatabaseError> {
        match self {
            Self::Database(database_error) => Some(database_error),
            _ => None,
        }
    }
}

impl From<Box<dyn Error>> for GameRepositoryError {
    fn from(value: Box<dyn Error>) -> Self {
        Self::unexpected(value)
    }
}

impl From<diesel::result::Error> for GameRepositoryError {
    fn from(error: diesel::result::Error) -> Self {
        match error {
            diesel::result::Error::DatabaseError(kind, info) => match kind {
                DatabaseErrorKind::ForeignKeyViolation => {
                    Self::database(GameDatabaseError::foreign_key_violation(info.message()))
                }
                DatabaseErrorKind::UniqueViolation => Self::database(
                    GameDatabaseError::unique_constraint_violation(info.message()),
                ),
                _ => Self::database(GameDatabaseError::other(info.message())),
            },
            diesel::result::Error::NotFound => Self::database(GameDatabaseError::not_found()),
            _ => Self::unexpected(error.into()),
        }
    }
}
