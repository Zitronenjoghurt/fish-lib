use crate::data::location_data::LocationData;
use crate::data::species_data::SpeciesData;
use crate::game::errors::GameResult;
use crate::models::fishing_history_entry::FishingHistoryEntry;
use crate::models::specimen::Specimen;
use crate::models::user::User;
use std::sync::Arc;

/// # Game Interface
///
/// The trait defining all available game operations. This interface is implemented
/// by the [`crate::game::Game`] struct to provide the actual game functionality. It serves as a
/// contract to ensure all required functionality is implemented and to prevent
/// accidental breaking changes.
pub trait GameInterface: Send + Sync {
    fn get_user(&self, external_id: i64) -> GameResult<User>;
    fn register_user(&self, external_id: i64) -> GameResult<User>;
    fn user_catch_specific_specimen(
        &self,
        user: &User,
        species_id: i32,
    ) -> GameResult<(Specimen, FishingHistoryEntry)>;
    fn user_get_fishing_history(
        &self,
        user: &User,
        species_id: i32,
    ) -> GameResult<FishingHistoryEntry>;
    fn get_location_data(&self, location_id: i32) -> GameResult<Arc<LocationData>>;
    fn get_species_data(&self, species_id: i32) -> GameResult<Arc<SpeciesData>>;
}
