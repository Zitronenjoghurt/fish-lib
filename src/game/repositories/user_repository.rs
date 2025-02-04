use crate::database::DatabaseInterface;
use crate::game::errors::repository::GameRepositoryError;
use crate::models::user::{NewUser, User};
use crate::schema::fish_users;
use crate::traits::repository::Repository;
use chrono::Utc;
use diesel::prelude::*;
use std::sync::{Arc, RwLock};

pub trait UserRepositoryInterface: Repository<User> + Send + Sync {
    fn find_by_external_id(&self, external_id: i64) -> Result<Option<User>, GameRepositoryError>;
}

pub struct UserRepository {
    db: Arc<RwLock<dyn DatabaseInterface>>,
}

impl UserRepository {
    pub fn new(db: Arc<RwLock<dyn DatabaseInterface>>) -> Self {
        Self { db }
    }
}

impl UserRepositoryInterface for UserRepository {
    fn find_by_external_id(&self, external_id: i64) -> Result<Option<User>, GameRepositoryError> {
        let mut connection = self.get_connection()?;
        let user = fish_users::table
            .filter(fish_users::external_id.eq(external_id))
            .first::<User>(&mut connection)
            .optional()?;
        Ok(user)
    }
}

impl Repository<User> for UserRepository {
    fn get_db(&self) -> Arc<RwLock<dyn DatabaseInterface>> {
        self.db.clone()
    }

    fn create(&self, new_entity: NewUser) -> Result<User, GameRepositoryError> {
        let mut connection = self.get_connection()?;

        let new_result = diesel::insert_into(fish_users::table)
            .values(new_entity)
            .get_result::<User>(&mut connection)?;

        Ok(new_result)
    }

    fn find(&self, id: i64) -> Result<Option<User>, GameRepositoryError> {
        let mut connection = self.get_connection()?;
        let user = fish_users::table
            .find(id)
            .first::<User>(&mut connection)
            .optional()?;
        Ok(user)
    }

    fn save(&self, mut entity: User) -> Result<User, GameRepositoryError> {
        let mut connection = self.get_connection()?;
        entity.updated_at = Utc::now();

        let updated_user = diesel::update(fish_users::table)
            .filter(fish_users::id.eq(entity.id))
            .set(entity)
            .get_result::<User>(&mut connection)?;

        Ok(updated_user)
    }

    fn delete(&self, entity: User) -> Result<bool, GameRepositoryError> {
        let mut connection = self.get_connection()?;

        let deleted_count = diesel::delete(fish_users::table)
            .filter(fish_users::id.eq(entity.id))
            .execute(&mut connection)?;

        Ok(deleted_count > 0)
    }
}
