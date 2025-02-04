use crate::database::DatabaseInterface;
use crate::game::errors::repository::GameRepositoryError;
use crate::models::pond::{NewPond, Pond};
use crate::models::user::User;
use crate::schema::fish_ponds;
use crate::traits::repository::Repository;
use chrono::Utc;
use diesel::prelude::*;
use std::sync::{Arc, RwLock};

pub trait PondRepositoryInterface: Repository<Pond> + Send + Sync {
    fn find_by_user(&self, owner_user: &User) -> Result<Vec<Pond>, GameRepositoryError>;
}

pub struct PondRepository {
    db: Arc<RwLock<dyn DatabaseInterface>>,
}

impl PondRepository {
    pub fn new(db: Arc<RwLock<dyn DatabaseInterface>>) -> Self {
        PondRepository { db }
    }
}

impl PondRepositoryInterface for PondRepository {
    fn find_by_user(&self, owner_user: &User) -> Result<Vec<Pond>, GameRepositoryError> {
        let mut connection = self.get_connection()?;

        let pond = fish_ponds::table
            .filter(fish_ponds::user_id.eq(owner_user.id))
            .load::<Pond>(&mut connection)?;

        Ok(pond)
    }
}

impl Repository<Pond> for PondRepository {
    fn get_db(&self) -> Arc<RwLock<dyn DatabaseInterface>> {
        self.db.clone()
    }

    fn create(&self, new_entity: NewPond) -> Result<Pond, GameRepositoryError> {
        let mut connection = self.get_connection()?;

        let new_result = diesel::insert_into(fish_ponds::table)
            .values(new_entity)
            .get_result::<Pond>(&mut connection)?;

        Ok(new_result)
    }

    fn find(&self, id: i64) -> Result<Option<Pond>, GameRepositoryError> {
        let mut connection = self.get_connection()?;
        let pond = fish_ponds::table
            .find(id)
            .first::<Pond>(&mut connection)
            .optional()?;
        Ok(pond)
    }

    fn save(&self, mut entity: Pond) -> Result<Pond, GameRepositoryError> {
        let mut connection = self.get_connection()?;
        entity.updated_at = Utc::now();

        let updated_pond = diesel::update(fish_ponds::table)
            .filter(fish_ponds::id.eq(entity.id))
            .set(entity)
            .get_result::<Pond>(&mut connection)?;

        Ok(updated_pond)
    }

    fn delete(&self, entity: &Pond) -> Result<bool, GameRepositoryError> {
        let mut connection = self.get_connection()?;

        let deleted_count = diesel::delete(fish_ponds::table)
            .filter(fish_ponds::id.eq(entity.id))
            .execute(&mut connection)?;

        Ok(deleted_count > 0)
    }
}
