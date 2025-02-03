use crate::data::location_data::LocationData;
use crate::data::species_data::SpeciesData;
use crate::game::errors::GameResult;
use crate::models::user::User;
use std::sync::Arc;

/// # Game Interface
///
/// The trait defining all available game operations. This interface is implemented
/// by the [`crate::game::Game`] struct to provide the actual game functionality. It serves as a
/// contract to ensure all required functionality is implemented and to prevent
/// accidental breaking changes.
pub trait GameInterface {
    fn register_user(external_id: i64) -> GameResult<User>;
    fn get_user(external_id: i64) -> GameResult<User>;
    fn get_location_data(location_id: i32) -> GameResult<Arc<LocationData>>;
    fn get_species_data(species_id: i32) -> GameResult<Arc<SpeciesData>>;
}
