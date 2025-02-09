use crate::data::location_data::LocationData;
use crate::data::species_data::SpeciesData;
use crate::dto::user_location_unlock::UserLocationUnlock;
use crate::game::errors::GameResult;
use crate::game::systems::weather_system::weather::Weather;
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
    fn location_find(&self, location_id: i32) -> GameResult<Arc<LocationData>>;
    fn location_weather_current(&self, location: Arc<LocationData>) -> GameResult<Weather>;
    fn species_find(&self, species_id: i32) -> GameResult<Arc<SpeciesData>>;
    fn user_catch_specific_specimen(
        &self,
        user: &User,
        species: Arc<SpeciesData>,
    ) -> GameResult<(Specimen, FishingHistoryEntry)>;
    fn user_get_fishing_history(
        &self,
        user: &User,
        species: Arc<SpeciesData>,
    ) -> GameResult<FishingHistoryEntry>;
    fn user_find(&self, external_id: i64) -> GameResult<User>;
    fn user_get_unlocked_locations(&self, user: &User) -> GameResult<Vec<UserLocationUnlock>>;
    fn user_register(&self, external_id: i64) -> GameResult<User>;
    fn user_save(&self, user: User) -> GameResult<User>;
    fn user_unlock_location(
        &self,
        user: &User,
        location: Arc<LocationData>,
    ) -> GameResult<UserLocationUnlock>;
}
