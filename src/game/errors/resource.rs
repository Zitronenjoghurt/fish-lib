use thiserror::Error;

#[derive(Error, Debug)]
pub enum GameResourceError {
    #[error("Location with id '{location_id}' does not exist")]
    LocationNotFound { location_id: i32 },
    #[error("Species with id '{species_id}' does not exist")]
    SpeciesNotFound { species_id: i32 },
    #[error("User with external id '{external_id}' already exists")]
    UserAlreadyExists { external_id: i64 },
    #[error("User with external id '{external_id}' does not exist")]
    UserNotFound { external_id: i64 },
}

impl GameResourceError {
    pub fn location_not_found(location_id: i32) -> Self {
        Self::LocationNotFound { location_id }
    }

    pub fn species_not_found(species_id: i32) -> Self {
        Self::SpeciesNotFound { species_id }
    }

    pub fn user_already_exists(external_id: i64) -> Self {
        Self::UserAlreadyExists { external_id }
    }

    pub fn user_not_found(external_id: i64) -> Self {
        Self::UserNotFound { external_id }
    }

    pub fn is_location_not_found(&self) -> bool {
        matches!(self, Self::LocationNotFound { .. })
    }

    pub fn is_species_not_found(&self) -> bool {
        matches!(self, Self::SpeciesNotFound { .. })
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

    pub fn get_location_id(&self) -> Option<i32> {
        match self {
            Self::LocationNotFound { location_id } => Some(*location_id),
            _ => None,
        }
    }

    pub fn get_species_id(&self) -> Option<i32> {
        match self {
            Self::SpeciesNotFound { species_id } => Some(*species_id),
            _ => None,
        }
    }
}
