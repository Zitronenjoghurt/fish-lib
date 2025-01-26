use crate::get_config;
use crate::traits::model::Model;
use crate::utils::random::random_normal;
use chrono::{DateTime, Utc};
use diesel::{AsChangeset, Insertable, Queryable, Selectable};
use rand::random;
use std::error::Error;

#[derive(Debug, Clone, PartialEq, Queryable, Selectable, AsChangeset)]
#[diesel(table_name = crate::schema::fish_fishes)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Fish {
    pub id: i64,
    pub user_id: i64,
    pub species_id: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub size_baby_mm: f32,
    pub size_adult_mm: f32,
    pub lifespan_days: f32,
    /// The age this fish was caught at (from 0 to 1)
    pub catch_age: f32,
}

impl Fish {
    pub fn get_age(&self, time_multiplier: f32) -> f32 {
        if self.catch_age >= 1.0 {
            return 1.0;
        }

        let now = Utc::now();
        let seconds_since_catch = (now - self.created_at).num_seconds();
        let days_since_catch = seconds_since_catch as f32 / (86400f32);

        let remaining_lifespan_days_after_catch = self.lifespan_days * (1.0 - self.catch_age);
        let age_progress_since_catch =
            (days_since_catch * time_multiplier) / remaining_lifespan_days_after_catch;

        (self.catch_age + age_progress_since_catch).clamp(0.0, 1.0)
    }

    pub fn get_size_mm(&self, time_multiplier: f32) -> f32 {
        let current_age = self.get_age(time_multiplier);
        self.size_baby_mm + (self.size_adult_mm - self.size_baby_mm) * current_age
    }
}

impl Model for Fish {
    type Table = crate::schema::fish_fishes::table;
    type PrimaryKeyType = i64;
    type InsertType = NewFish;

    fn table() -> Self::Table {
        crate::schema::fish_fishes::table
    }

    fn id(&self) -> Self::PrimaryKeyType {
        self.id
    }
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::fish_fishes)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewFish {
    pub user_id: i64,
    pub species_id: i32,
    pub size_baby_mm: f32,
    pub size_adult_mm: f32,
    pub lifespan_days: f32,
    pub catch_age: f32,
}

impl NewFish {
    pub fn generate(user_id: i64, species_id: i32) -> Result<NewFish, Box<dyn Error>> {
        let data = get_config()
            .get_fish_data(species_id)
            .ok_or_else(|| format!("Fish data with species id '{}' does not exist.", species_id))?;

        let size_baby_mm =
            random_normal(data.min_size_baby_mm as f32, data.max_size_baby_mm as f32);
        let size_adult_mm =
            random_normal(data.min_size_adult_mm as f32, data.max_size_adult_mm as f32);
        let lifespan_days =
            random_normal(data.min_lifespan_days as f32, data.max_lifespan_days as f32);

        let fish = NewFish {
            user_id,
            catch_age: random(),
            species_id,
            size_baby_mm,
            size_adult_mm,
            lifespan_days,
        };

        Ok(fish)
    }
}
