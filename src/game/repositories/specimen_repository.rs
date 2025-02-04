use crate::database::DatabaseInterface;
use crate::game::errors::repository::GameRepositoryError;
use crate::models::specimen::{NewSpecimen, Specimen};
use crate::models::user::User;
use crate::schema::fish_specimens;
use crate::traits::repository::Repository;
use chrono::Utc;
use diesel::prelude::*;
use std::sync::{Arc, RwLock};

pub trait SpecimenRepositoryInterface: Repository<Specimen> + Send + Sync {
    fn find_by_user(&self, owner_user: &User) -> Result<Vec<Specimen>, GameRepositoryError>;
}

pub struct SpecimenRepository {
    db: Arc<RwLock<dyn DatabaseInterface>>,
}

impl SpecimenRepository {
    pub fn new(db: Arc<RwLock<dyn DatabaseInterface>>) -> Self {
        Self { db }
    }
}

impl SpecimenRepositoryInterface for SpecimenRepository {
    fn find_by_user(&self, owner_user: &User) -> Result<Vec<Specimen>, GameRepositoryError> {
        let mut connection = self.get_connection()?;

        let specimens = fish_specimens::table
            .filter(fish_specimens::user_id.eq(owner_user.id))
            .load::<Specimen>(&mut connection)?;

        Ok(specimens)
    }
}

impl Repository<Specimen> for SpecimenRepository {
    fn get_db(&self) -> Arc<RwLock<dyn DatabaseInterface>> {
        self.db.clone()
    }

    fn create(&self, new_entity: NewSpecimen) -> Result<Specimen, GameRepositoryError> {
        let mut connection = self.get_connection()?;

        let specimen = diesel::insert_into(fish_specimens::table)
            .values(new_entity)
            .get_result::<Specimen>(&mut connection)?;

        Ok(specimen)
    }

    fn find(&self, id: i64) -> Result<Option<Specimen>, GameRepositoryError> {
        let mut connection = self.get_connection()?;
        let specimen = fish_specimens::table
            .find(id)
            .first::<Specimen>(&mut connection)
            .optional()?;
        Ok(specimen)
    }

    fn save(&self, mut entity: Specimen) -> Result<Specimen, GameRepositoryError> {
        let mut connection = self.get_connection()?;
        entity.updated_at = Utc::now();

        let updated_specimen = diesel::update(fish_specimens::table)
            .filter(fish_specimens::id.eq(entity.id))
            .set(entity)
            .get_result::<Specimen>(&mut connection)?;

        Ok(updated_specimen)
    }

    fn delete(&self, entity: Specimen) -> Result<bool, GameRepositoryError> {
        let mut connection = self.get_connection()?;

        let deleted_count = diesel::delete(fish_specimens::table)
            .filter(fish_specimens::id.eq(entity.id))
            .execute(&mut connection)?;

        Ok(deleted_count > 0)
    }
}
