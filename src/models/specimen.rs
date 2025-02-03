use crate::get_config;
use crate::traits::model::Model;
use crate::utils::random::random_normal_01;
use chrono::{DateTime, Utc};
use diesel::{AsChangeset, Insertable, Queryable, Selectable};
use rand::random;
use std::error::Error;

#[derive(Debug, Clone, PartialEq, Queryable, Selectable, AsChangeset)]
#[diesel(table_name = crate::schema::fish_specimens)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Specimen {
    pub id: i64,
    pub user_id: i64,
    pub species_id: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub size_baby_ratio: f32,
    pub size_adult_ratio: f32,
    pub lifespan_days_ratio: f32,
    /// The age this fish was caught at (from 0 to 1)
    pub catch_age: f32,
}

impl Specimen {
    pub fn get_age(&self, time_multiplier: f32) -> f32 {
        let data = get_config()
            .get_species_data(self.species_id)
            .unwrap_or_else(|| panic!("Missing fish data for species id '{}'", self.species_id));
        let lifespan_days = data.get_lifespan_days_by_ratio(self.lifespan_days_ratio);

        if self.catch_age >= 1.0 {
            return 1.0;
        }

        let now = Utc::now();
        let seconds_since_catch = (now - self.created_at).num_seconds();
        let days_since_catch = seconds_since_catch as f32 / (86400f32);

        let remaining_lifespan_days_after_catch = lifespan_days * (1.0 - self.catch_age);
        let age_progress_since_catch =
            (days_since_catch * time_multiplier) / remaining_lifespan_days_after_catch;

        (self.catch_age + age_progress_since_catch).clamp(0.0, 1.0)
    }

    pub fn get_size_mm(&self, time_multiplier: f32) -> f32 {
        let data = get_config()
            .get_species_data(self.species_id)
            .unwrap_or_else(|| panic!("Missing fish data for species id '{}'", self.species_id));
        let size_baby_mm = data.get_baby_size_by_ratio(self.size_baby_ratio);
        let size_adult_mm = data.get_adult_size_by_ratio(self.size_adult_ratio);

        let current_age = self.get_age(time_multiplier);
        size_baby_mm + (size_adult_mm - size_baby_mm) * current_age
    }

    pub fn get_weight_g(&self, time_multiplier: f32) -> f32 {
        let data = get_config()
            .get_species_data(self.species_id)
            .unwrap_or_else(|| panic!("Missing fish data for species id '{}'", self.species_id));
        let weight_baby_g = data.get_baby_weight_by_ratio(self.size_baby_ratio);
        let weight_adult_g = data.get_adult_weight_by_ratio(self.size_adult_ratio);

        let current_age = self.get_age(time_multiplier);
        weight_baby_g + (weight_adult_g - weight_baby_g) * current_age
    }

    pub fn get_total_size_ratio(&self, time_multiplier: f32) -> f32 {
        let data = get_config()
            .get_species_data(self.species_id)
            .unwrap_or_else(|| panic!("Missing fish data for species id '{}'", self.species_id));
        let min_possible_size = data.min_size_baby_mm as f32;
        let max_possible_size = data.max_size_adult_mm as f32;

        let current_size = self.get_size_mm(time_multiplier);

        let ratio = (current_size - min_possible_size) / (max_possible_size - min_possible_size);
        ratio.clamp(0.0, 1.0)
    }
}

impl Model for Specimen {
    type Table = crate::schema::fish_specimens::table;
    type PrimaryKeyType = i64;
    type InsertType = NewSpecimen;

    fn table() -> Self::Table {
        crate::schema::fish_specimens::table
    }

    fn id(&self) -> Self::PrimaryKeyType {
        self.id
    }
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::fish_specimens)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewSpecimen {
    pub user_id: i64,
    pub species_id: i32,
    pub size_baby_ratio: f32,
    pub size_adult_ratio: f32,
    pub lifespan_days_ratio: f32,
    pub catch_age: f32,
}

impl NewSpecimen {
    pub fn generate(user_id: i64, species_id: i32) -> Result<NewSpecimen, Box<dyn Error>> {
        let _ = get_config()
            .get_species_data(species_id)
            .ok_or_else(|| format!("Fish data with species id '{}' does not exist.", species_id))?;

        let specimen = NewSpecimen {
            user_id,
            catch_age: random(),
            species_id,
            size_baby_ratio: random_normal_01(),
            size_adult_ratio: random_normal_01(),
            lifespan_days_ratio: random_normal_01(),
        };

        Ok(specimen)
    }
}
