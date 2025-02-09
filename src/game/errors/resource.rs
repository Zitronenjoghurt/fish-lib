use thiserror::Error;

#[derive(Error, Debug)]
pub enum GameResourceError {
    #[error("User with id '{user_id}' has no fishing history with species with id '{species_id}'")]
    FishingHistoryNotFound { user_id: i64, species_id: i32 },
    #[error("User with external id '{external_id}' has already unlocked location with id '{location_id}'")]
    LocationAlreadyUnlocked { external_id: i64, location_id: i32 },
    #[error("Location with id '{location_id}' does not exist")]
    LocationNotFound { location_id: i32 },
    #[error("No available encounters for the specified conditions")]
    NoAvailableEncounters,
    #[error("User with external id '{external_id}' has no fishing history with species with id '{species_id}'")]
    NoFishingHistory { external_id: i64, species_id: i32 },
    #[error("Species with id '{species_id}' does not exist")]
    SpeciesNotFound { species_id: i32 },
    #[error(
        "Unable to unlock location with id '{location_id}' because of unmet unlock requirements"
    )]
    UnmetLocationUnlockRequirements { location_id: i32 },
    #[error("User with external id '{external_id}' already exists")]
    UserAlreadyExists { external_id: i64 },
    #[error("User with external id '{external_id}' does not exist")]
    UserNotFound { external_id: i64 },
}

impl GameResourceError {
    pub fn fishing_history_not_found(user_id: i64, species_id: i32) -> Self {
        Self::FishingHistoryNotFound {
            user_id,
            species_id,
        }
    }

    pub fn location_already_unlocked(external_id: i64, location_id: i32) -> Self {
        Self::LocationAlreadyUnlocked {
            external_id,
            location_id,
        }
    }

    pub fn location_not_found(location_id: i32) -> Self {
        Self::LocationNotFound { location_id }
    }

    pub fn no_available_encounters() -> Self {
        Self::NoAvailableEncounters
    }

    pub fn no_fishing_history(external_id: i64, species_id: i32) -> Self {
        Self::NoFishingHistory {
            external_id,
            species_id,
        }
    }

    pub fn species_not_found(species_id: i32) -> Self {
        Self::SpeciesNotFound { species_id }
    }

    pub fn unmet_location_unlock_requirements(location_id: i32) -> Self {
        Self::UnmetLocationUnlockRequirements { location_id }
    }

    pub fn user_already_exists(external_id: i64) -> Self {
        Self::UserAlreadyExists { external_id }
    }

    pub fn user_not_found(external_id: i64) -> Self {
        Self::UserNotFound { external_id }
    }

    pub fn is_fishing_history_not_found(&self) -> bool {
        matches!(self, Self::FishingHistoryNotFound { .. })
    }

    pub fn is_location_already_unlocked(&self) -> bool {
        matches!(self, Self::LocationAlreadyUnlocked { .. })
    }

    pub fn is_location_not_found(&self) -> bool {
        matches!(self, Self::LocationNotFound { .. })
    }

    pub fn is_no_available_encounters(&self) -> bool {
        matches!(self, Self::NoAvailableEncounters)
    }

    pub fn is_no_fishing_history(&self) -> bool {
        matches!(self, Self::NoFishingHistory { .. })
    }

    pub fn is_species_not_found(&self) -> bool {
        matches!(self, Self::SpeciesNotFound { .. })
    }

    pub fn is_unmet_location_unlock_requirements(&self) -> bool {
        matches!(self, Self::UnmetLocationUnlockRequirements { .. })
    }

    pub fn is_user_already_exists(&self) -> bool {
        matches!(self, Self::UserAlreadyExists { .. })
    }

    pub fn is_user_not_found(&self) -> bool {
        matches!(self, Self::UserNotFound { .. })
    }

    pub fn get_external_id(&self) -> Option<i64> {
        match self {
            Self::LocationAlreadyUnlocked { external_id, .. } => Some(*external_id),
            Self::NoFishingHistory { external_id, .. } => Some(*external_id),
            Self::UserAlreadyExists { external_id } => Some(*external_id),
            Self::UserNotFound { external_id } => Some(*external_id),
            _ => None,
        }
    }

    pub fn get_location_id(&self) -> Option<i32> {
        match self {
            Self::LocationAlreadyUnlocked { location_id, .. } => Some(*location_id),
            Self::LocationNotFound { location_id } => Some(*location_id),
            Self::UnmetLocationUnlockRequirements { location_id, .. } => Some(*location_id),
            _ => None,
        }
    }

    pub fn get_species_id(&self) -> Option<i32> {
        match self {
            Self::FishingHistoryNotFound { species_id, .. } => Some(*species_id),
            Self::NoFishingHistory { species_id, .. } => Some(*species_id),
            Self::SpeciesNotFound { species_id } => Some(*species_id),
            _ => None,
        }
    }

    pub fn get_user_id(&self) -> Option<i64> {
        match self {
            Self::FishingHistoryNotFound { user_id, .. } => Some(*user_id),
            _ => None,
        }
    }
}
