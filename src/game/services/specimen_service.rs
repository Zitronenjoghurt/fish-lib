use crate::config::ConfigInterface;
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
        species_id: i32,
    ) -> GameResult<Specimen>;

    fn process_catch(&self, user: &User, species_id: i32) -> GameResult<Specimen>;

    fn species_exists(&self, species_id: i32) -> bool;
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
        species_id: i32,
    ) -> GameResult<Specimen> {
        if !self.species_exists(species_id) {
            return Err(GameResourceError::species_not_found(species_id))?;
        }

        let new_fish = NewSpecimen::generate(owner_user.id, species_id);
        match self.specimen_repository.create(new_fish) {
            Ok(specimen) => Ok(specimen),
            Err(e) => {
                if let Some(database_error) = e.get_database_error() {
                    if database_error.is_foreign_key_violation() {
                        Err(GameResourceError::user_not_found(owner_user.external_id).into())
                    } else {
                        Err(e.into())
                    }
                } else {
                    Err(e.into())
                }
            }
        }
    }

    fn process_catch(&self, user: &User, species_id: i32) -> GameResult<Specimen> {
        let fish = self.generate_and_save_specimen(user, species_id)?;
        Ok(fish)
    }

    fn species_exists(&self, species_id: i32) -> bool {
        match self.config.get_species_data(species_id) {
            Some(_) => true,
            None => false,
        }
    }
}
