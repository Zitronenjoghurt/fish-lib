use crate::game::errors::repository::GameRepositoryError;
use crate::get_db_connection;
use crate::models::specimen::{NewSpecimen, Specimen};
use crate::models::user::User;
use crate::schema::fish_specimens;
use crate::traits::repository::Repository;
use chrono::Utc;
use diesel::prelude::*;
use diesel::result::DatabaseErrorKind;
use std::error::Error;

pub struct SpecimenRepository;

impl SpecimenRepository {
    pub fn find_by_user(owner_user: &User) -> Result<Vec<Specimen>, Box<dyn Error>> {
        let mut connection = get_db_connection()?;

        let specimens = fish_specimens::table
            .filter(fish_specimens::user_id.eq(owner_user.id))
            .load::<Specimen>(&mut connection)?;

        Ok(specimens)
    }
}

impl Repository<Specimen> for SpecimenRepository {
    fn create(new_entity: NewSpecimen) -> Result<Specimen, Box<dyn Error>> {
        let user_id = new_entity.user_id;
        let mut connection = get_db_connection()?;

        match diesel::insert_into(fish_specimens::table)
            .values(new_entity)
            .get_result::<Specimen>(&mut connection)
        {
            Ok(specimen) => Ok(specimen),
            Err(diesel::result::Error::DatabaseError(
                DatabaseErrorKind::ForeignKeyViolation,
                ..,
            )) => Err(GameRepositoryError::foreign_key_violation_user_not_found(user_id).into()),
            Err(e) => Err(e.into()),
        }
    }

    fn find(id: i64) -> Result<Option<Specimen>, Box<dyn Error>> {
        let mut connection = get_db_connection()?;
        let specimen = fish_specimens::table
            .find(id)
            .first::<Specimen>(&mut connection)
            .optional()?;
        Ok(specimen)
    }

    fn save(mut entity: Specimen) -> Result<Specimen, Box<dyn Error>> {
        let mut connection = get_db_connection()?;
        entity.updated_at = Utc::now();

        let updated_specimen = diesel::update(fish_specimens::table)
            .filter(fish_specimens::id.eq(entity.id))
            .set(entity)
            .get_result::<Specimen>(&mut connection)?;

        Ok(updated_specimen)
    }

    fn delete(entity: &Specimen) -> Result<bool, Box<dyn Error>> {
        let mut connection = get_db_connection()?;

        let deleted_count = diesel::delete(fish_specimens::table)
            .filter(fish_specimens::id.eq(entity.id))
            .execute(&mut connection)?;

        Ok(deleted_count > 0)
    }
}
