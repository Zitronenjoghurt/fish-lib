use thiserror::Error;

#[derive(Error, Debug)]
pub enum GameRepositoryError {
    #[error("Foreign key violation: user with id '{id}' was not found")]
    ForeignKeyViolationUserNotFound { id: i64 },
}

impl GameRepositoryError {
    pub fn foreign_key_violation_user_not_found(id: i64) -> Self {
        Self::ForeignKeyViolationUserNotFound { id }
    }

    pub fn is_foreign_key_violation_user_not_found(&self) -> bool {
        matches!(self, Self::ForeignKeyViolationUserNotFound { .. })
    }
}
