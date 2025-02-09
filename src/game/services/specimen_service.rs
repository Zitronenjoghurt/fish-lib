use crate::config::ConfigInterface;
use crate::data::species_data::SpeciesData;
use crate::game::errors::resource::GameResourceError;
use crate::game::errors::GameResult;
use crate::game::repositories::specimen_repository::SpecimenRepositoryInterface;
use crate::models::specimen::{NewSpecimen, Specimen};
use crate::models::user::User;
use std::sync::Arc;

pub trait SpecimenServiceInterface: Send + Sync {
    fn generate_and_save_specimen(
        &self,
        owner_user: &User,
        species_data: Arc<SpeciesData>,
    ) -> GameResult<Specimen>;

    fn process_catch(&self, user: &User, species_data: Arc<SpeciesData>) -> GameResult<Specimen>;
}

pub struct SpecimenService {
    config: Arc<dyn ConfigInterface>,
    specimen_repository: Arc<dyn SpecimenRepositoryInterface>,
}

impl SpecimenService {
    pub fn new(
        config: Arc<dyn ConfigInterface>,
        specimen_repository: Arc<dyn SpecimenRepositoryInterface>,
    ) -> Self {
        Self {
            config,
            specimen_repository,
        }
    }
}

impl SpecimenServiceInterface for SpecimenService {
    fn generate_and_save_specimen(
        &self,
        owner_user: &User,
        species_data: Arc<SpeciesData>,
    ) -> GameResult<Specimen> {
        let new_fish = NewSpecimen::generate(owner_user.id, species_data.id);
        self.specimen_repository
            .create(new_fish)
            .map_err(|e| match e.get_database_error() {
                Some(db_error) if db_error.is_foreign_key_violation() => {
                    GameResourceError::user_not_found(owner_user.external_id).into()
                }
                _ => e.into(),
            })
    }

    fn process_catch(&self, user: &User, species_data: Arc<SpeciesData>) -> GameResult<Specimen> {
        let fish = self.generate_and_save_specimen(user, species_data)?;
        Ok(fish)
    }
}
