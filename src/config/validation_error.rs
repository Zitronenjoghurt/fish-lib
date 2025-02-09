use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigValidationError {
    #[error("Invalid encounter location_id '{location_id}' for species with id '{species_id}'")]
    InvalidEncounterLocation { species_id: i32, location_id: i32 },
}

impl ConfigValidationError {
    pub fn invalid_encounter_location(species_id: i32, location_id: i32) -> Self {
        Self::InvalidEncounterLocation {
            species_id,
            location_id,
        }
    }

    pub fn is_invalid_encounter_location(&self) -> bool {
        matches!(self, Self::InvalidEncounterLocation { .. })
    }

    pub fn get_species_id(&self) -> Option<i32> {
        match self {
            Self::InvalidEncounterLocation { species_id, .. } => Some(*species_id),
            _ => None,
        }
    }

    pub fn get_location_id(&self) -> Option<i32> {
        match self {
            Self::InvalidEncounterLocation { location_id, .. } => Some(*location_id),
            _ => None,
        }
    }
}
