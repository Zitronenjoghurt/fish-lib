use crate::database::DatabaseInterface;
use crate::game::errors::database::GameDatabaseError;
use crate::game::errors::repository::GameRepositoryError;
use crate::traits::model::Model;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use diesel::PgConnection;
use std::sync::{Arc, RwLock};

pub trait Repository<T: Model>: Send + Sync {
    fn get_db(&self) -> Arc<RwLock<dyn DatabaseInterface>>;
    fn create(&self, new_entity: T::InsertType) -> Result<T, GameRepositoryError>;

    fn find(&self, id: T::PrimaryKeyType) -> Result<Option<T>, GameRepositoryError>;
    fn save(&self, entity: T) -> Result<T, GameRepositoryError>;
    fn delete(&self, entity: &T) -> Result<bool, GameRepositoryError>;
    fn get_connection(
        &self,
    ) -> Result<PooledConnection<ConnectionManager<PgConnection>>, GameDatabaseError> {
        self.get_db()
            .read()
            .expect("Failed to get read lock on DB")
            .get_connection()
    }
}
