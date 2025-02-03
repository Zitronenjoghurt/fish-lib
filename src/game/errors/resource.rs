use thiserror::Error;

#[derive(Error, Debug)]
pub enum GameResourceError {
    #[error("User with external id '{external_id}' already exists")]
    UserAlreadyExists { external_id: i64 },
    #[error("User with external id '{external_id}' does not exist")]
    UserNotFound { external_id: i64 },
}

impl GameResourceError {
    pub fn user_already_exists(external_id: i64) -> Self {
        Self::UserAlreadyExists { external_id }
    }

    pub fn user_not_found(external_id: i64) -> Self {
        Self::UserNotFound { external_id }
    }

    pub fn is_user_already_exists(&self) -> bool {
        matches!(self, Self::UserAlreadyExists { .. })
    }

    pub fn is_user_not_found(&self) -> bool {
        matches!(self, Self::UserNotFound { .. })
    }

    pub fn get_external_id(&self) -> Option<i64> {
        match self {
            Self::UserAlreadyExists { external_id } => Some(*external_id),
            Self::UserNotFound { external_id } => Some(*external_id),
            _ => None,
        }
    }
}
