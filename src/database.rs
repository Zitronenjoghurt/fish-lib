use crate::game::errors::database::GameDatabaseError;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::PgConnection;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

pub struct Database {
    connection_pool: Option<Pool<ConnectionManager<PgConnection>>>,
}

impl Default for Database {
    fn default() -> Self {
        Self::new()
    }
}

impl Database {
    pub fn new() -> Self {
        Self {
            connection_pool: None,
        }
    }

    pub fn connect(&mut self, postgres_url: &str) -> Result<(), GameDatabaseError> {
        let connection_manager = ConnectionManager::<PgConnection>::new(postgres_url);
        let pool = Pool::builder()
            .build(connection_manager)
            .map_err(|e| GameDatabaseError::connection_failed(&e.to_string()))?;
        self.connection_pool = Some(pool);
        self.run_migrations()?;
        Ok(())
    }

    pub fn run_migrations(&self) -> Result<(), GameDatabaseError> {
        let mut connection = self.get_connection()?;
        connection
            .run_pending_migrations(MIGRATIONS)
            .map_err(|e| GameDatabaseError::migrations_failed(&e.to_string()))?;
        Ok(())
    }

    pub fn get_connection(
        &self,
    ) -> Result<PooledConnection<ConnectionManager<PgConnection>>, GameDatabaseError> {
        match &self.connection_pool {
            Some(pool) => pool
                .get()
                .map_err(|e| GameDatabaseError::connection_failed(&e.to_string())),
            None => Err(GameDatabaseError::missing_connection()),
        }
    }

    pub fn clear(&self) -> Result<(), GameDatabaseError> {
        let mut connection = self.get_connection()?;

        connection
            .revert_all_migrations(MIGRATIONS)
            .map_err(|e| GameDatabaseError::migrations_failed(&e.to_string()))?;

        connection
            .run_pending_migrations(MIGRATIONS)
            .map_err(|e| GameDatabaseError::migrations_failed(&e.to_string()))?;

        Ok(())
    }
}
