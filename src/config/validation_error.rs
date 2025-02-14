use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigValidationError {
    #[error("Item (ID: {source_item_id}): max_count has to be greater or equal 1")]
    ItemInvalidMaxCount { source_item_id: i32 },
    #[error("Item (ID: {source_item_id}): stackable items must have a max_count of 1")]
    ItemNonUniqueNotStackable { source_item_id: i32 },
    #[error("Location (ID: {source_location_id}): Invalid required_locations_unlocked location_id '{target_location_id}'")]
    LocationRequiredLocation {
        source_location_id: i32,
        target_location_id: i32,
    },
    #[error("Location (ID: {source_location_id}: Invalid required_species_caught species_id '{target_species_id}')'")]
    LocationRequiredSpecies {
        source_location_id: i32,
        target_species_id: i32,
    },
    #[error(
        "Species (ID: {source_species_id}): Invalid encounter location_id '{target_location_id}'"
    )]
    SpeciesEncounterLocation {
        source_species_id: i32,
        target_location_id: i32,
    },
}

impl ConfigValidationError {
    pub fn item_invalid_max_count(source_item_id: i32) -> Self {
        Self::ItemInvalidMaxCount { source_item_id }
    }

    pub fn item_non_unique_not_stackable(source_item_id: i32) -> Self {
        Self::ItemNonUniqueNotStackable { source_item_id }
    }

    pub fn location_required_location(source_location_id: i32, target_location_id: i32) -> Self {
        Self::LocationRequiredLocation {
            source_location_id,
            target_location_id,
        }
    }

    pub fn location_required_species(source_location_id: i32, target_species_id: i32) -> Self {
        Self::LocationRequiredSpecies {
            source_location_id,
            target_species_id,
        }
    }

    pub fn species_encounter_location(source_species_id: i32, target_location_id: i32) -> Self {
        Self::SpeciesEncounterLocation {
            source_species_id,
            target_location_id,
        }
    }

    pub fn is_item_invalid_max_count(&self) -> bool {
        matches!(self, Self::ItemInvalidMaxCount { .. })
    }

    pub fn is_item_non_unique_not_stackable(&self) -> bool {
        matches!(self, Self::ItemNonUniqueNotStackable { .. })
    }

    pub fn is_location_required_location(&self) -> bool {
        matches!(self, Self::LocationRequiredLocation { .. })
    }

    pub fn is_location_required_species(&self) -> bool {
        matches!(self, Self::LocationRequiredSpecies { .. })
    }

    pub fn is_species_encounter_location(&self) -> bool {
        matches!(self, Self::SpeciesEncounterLocation { .. })
    }

    pub fn get_source_species_id(&self) -> Option<i32> {
        match self {
            Self::SpeciesEncounterLocation {
                source_species_id, ..
            } => Some(*source_species_id),
            _ => None,
        }
    }

    pub fn get_target_species_id(&self) -> Option<i32> {
        match self {
            Self::LocationRequiredSpecies {
                target_species_id, ..
            } => Some(*target_species_id),
            _ => None,
        }
    }

    pub fn get_source_location_id(&self) -> Option<i32> {
        match self {
            Self::LocationRequiredLocation {
                source_location_id, ..
            } => Some(*source_location_id),
            Self::LocationRequiredSpecies {
                source_location_id, ..
            } => Some(*source_location_id),
            _ => None,
        }
    }

    pub fn get_target_location_id(&self) -> Option<i32> {
        match self {
            Self::SpeciesEncounterLocation {
                target_location_id, ..
            } => Some(*target_location_id),
            Self::LocationRequiredLocation {
                target_location_id, ..
            } => Some(*target_location_id),
            _ => None,
        }
    }

    pub fn get_source_item_id(&self) -> Option<i32> {
        match self {
            Self::ItemInvalidMaxCount { source_item_id, .. } => Some(*source_item_id),
            Self::ItemNonUniqueNotStackable { source_item_id, .. } => Some(*source_item_id),
            _ => None,
        }
    }
}
