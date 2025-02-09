use crate::game::errors::database::GameDatabaseError;
use crate::game::errors::repository::GameRepositoryError;
use crate::game::errors::resource::GameResourceError;
use thiserror::Error;

pub mod database;
pub mod repository;
pub mod resource;

pub type GameResult<T> = Result<T, GameError>;

#[derive(Error, Debug)]
pub enum GameError {
    #[error(transparent)]
    Database(#[from] GameDatabaseError),
    #[error(transparent)]
    Repository(#[from] GameRepositoryError),
    #[error(transparent)]
    Resource(#[from] GameResourceError),
    #[error("Unexpected error: {msg}")]
    Unexpected {
        msg: String,
        #[source]
        source: Box<dyn std::error::Error>,
    },
}

impl GameError {
    pub fn unexpected(error: Box<dyn std::error::Error>) -> Self {
        Self::Unexpected {
            msg: error.to_string(),
            source: error,
        }
    }

    pub fn as_database_error(&self) -> Option<&GameDatabaseError> {
        match self {
            Self::Database(e) => Some(e),
            _ => None,
        }
    }

    pub fn as_repository_error(&self) -> Option<&GameRepositoryError> {
        match self {
            Self::Repository(e) => Some(e),
            _ => None,
        }
    }

    pub fn as_resource_error(&self) -> Option<&GameResourceError> {
        match self {
            Self::Resource(e) => Some(e),
            _ => None,
        }
    }

    pub fn is_database_error(&self) -> bool {
        matches!(self, Self::Database(_))
    }

    pub fn is_repository_error(&self) -> bool {
        matches!(self, Self::Repository(_))
    }

    pub fn is_resource_error(&self) -> bool {
        matches!(self, Self::Resource(_))
    }

    pub fn is_already_exists(&self) -> bool {
        matches!(
            self,
            Self::Resource(GameResourceError::UserAlreadyExists { .. })
                | Self::Resource(GameResourceError::LocationAlreadyUnlocked { .. })
        )
    }

    pub fn is_not_found(&self) -> bool {
        matches!(
            self,
            Self::Resource(GameResourceError::UserNotFound { .. })
                | Self::Resource(GameResourceError::FishingHistoryNotFound { .. })
                | Self::Resource(GameResourceError::LocationNotFound { .. })
                | Self::Resource(GameResourceError::SpeciesNotFound { .. })
                | Self::Resource(GameResourceError::NoFishingHistory { .. })
                | Self::Repository(GameRepositoryError::Database(GameDatabaseError::NotFound))
        )
    }
}

impl From<Box<dyn std::error::Error>> for GameError {
    fn from(error: Box<dyn std::error::Error>) -> Self {
        match error {
            e if e.is::<GameError>() => *e.downcast::<GameError>().unwrap(),
            e if e.is::<GameDatabaseError>() => {
                GameError::Database(*e.downcast::<GameDatabaseError>().unwrap())
            }
            e if e.is::<GameRepositoryError>() => {
                GameError::Repository(*e.downcast::<GameRepositoryError>().unwrap())
            }
            e if e.is::<GameResourceError>() => {
                GameError::Resource(*e.downcast::<GameResourceError>().unwrap())
            }
            e => Self::unexpected(e),
        }
    }
}
