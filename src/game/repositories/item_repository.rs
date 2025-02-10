use crate::database::DatabaseInterface;
use crate::game::errors::repository::GameRepositoryError;
use crate::models::item::{Item, NewItem};
use crate::schema::fish_items;
use crate::traits::repository::Repository;
use chrono::Utc;
use diesel::prelude::*;
use std::sync::{Arc, RwLock};

pub trait ItemRepositoryInterface: Repository<Item> + Send + Sync {}

pub struct ItemRepository {
    db: Arc<RwLock<dyn DatabaseInterface>>,
}

impl ItemRepository {
    pub fn new(db: Arc<RwLock<dyn DatabaseInterface>>) -> Self {
        Self { db }
    }
}

impl ItemRepositoryInterface for ItemRepository {}

impl Repository<Item> for ItemRepository {
    fn get_db(&self) -> Arc<RwLock<dyn DatabaseInterface>> {
        self.db.clone()
    }

    fn create(&self, new_entity: NewItem) -> Result<Item, GameRepositoryError> {
        let mut connection = self.get_connection()?;

        let new_result = diesel::insert_into(fish_items::table)
            .values(new_entity)
            .get_result::<Item>(&mut connection)?;

        Ok(new_result)
    }

    fn find(&self, id: i64) -> Result<Option<Item>, GameRepositoryError> {
        let mut connection = self.get_connection()?;
        let item = fish_items::table
            .find(id)
            .first::<Item>(&mut connection)
            .optional()?;
        Ok(item)
    }

    fn save(&self, mut entity: Item) -> Result<Item, GameRepositoryError> {
        let mut connection = self.get_connection()?;
        entity.updated_at = Utc::now();

        let updated_item = diesel::update(fish_items::table)
            .filter(fish_items::id.eq(entity.id))
            .set(entity)
            .get_result::<Item>(&mut connection)?;

        Ok(updated_item)
    }

    fn delete(&self, entity: Item) -> Result<bool, GameRepositoryError> {
        let mut connection = self.get_connection()?;

        let deleted_count = diesel::delete(fish_items::table)
            .filter(fish_items::id.eq(entity.id))
            .execute(&mut connection)?;

        Ok(deleted_count > 0)
    }
}
