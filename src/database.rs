use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::sql_types::Text;
use diesel::{PgConnection, QueryableByName, RunQueryDsl};
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

    pub fn connect(&mut self, postgres_url: &str) -> Result<(), Box<dyn std::error::Error>> {
        let connection_manager = ConnectionManager::<PgConnection>::new(postgres_url);
        let pool = Pool::builder().build(connection_manager)?;
        self.connection_pool = Some(pool);
        self.run_migrations()?;
        Ok(())
    }

    pub fn run_migrations(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut connection = self.get_connection()?;
        connection
            .run_pending_migrations(MIGRATIONS)
            .map_err(|e| format!("Failed to run migrations: {}", e))?;
        Ok(())
    }

    pub fn get_connection(
        &self,
    ) -> Result<PooledConnection<ConnectionManager<PgConnection>>, Box<dyn std::error::Error>> {
        self.connection_pool
            .as_ref()
            .and_then(|pool| pool.get().ok())
            .ok_or_else(|| "Failed to get database connection".into())
    }

    pub fn clear(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut connection = self.get_connection()?;

        println!("Attempting to clear database...");

        #[derive(QueryableByName, Debug)]
        struct TableName {
            #[diesel(sql_type = Text)]
            tablename: String,
        }

        let table_names: Vec<String> =
            diesel::sql_query("SELECT tablename FROM pg_tables WHERE schemaname = 'public'")
                .load::<TableName>(&mut connection)?
                .into_iter()
                .map(|table| table.tablename)
                .collect();

        println!("Tables to truncate: {:?}", table_names);

        for table in table_names {
            println!("Truncating table: {}", table);
            match diesel::sql_query(format!("TRUNCATE TABLE {} RESTART IDENTITY CASCADE", table))
                .execute(&mut connection)
            {
                Ok(_) => println!("Successfully truncated {}", table),
                Err(e) => println!("Error truncating {}: {:?}", table, e),
            }
        }

        Ok(())
    }
}
