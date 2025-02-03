use crate::game::errors::database::GameDatabaseError;
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

    pub fn as_resource_error(&self) -> Option<&GameResourceError> {
        match self {
            GameError::Resource(e) => Some(e),
            _ => None,
        }
    }
}

impl From<Box<dyn std::error::Error>> for GameError {
    fn from(error: Box<dyn std::error::Error>) -> Self {
        match error {
            e if e.is::<GameError>() => *e.downcast::<GameError>().unwrap(),
            e if e.is::<GameDatabaseError>() => {
                GameError::Database(*e.downcast::<GameDatabaseError>().unwrap())
            }
            e if e.is::<GameResourceError>() => {
                GameError::Resource(*e.downcast::<GameResourceError>().unwrap())
            }
            e => Self::unexpected(e),
        }
    }
}
