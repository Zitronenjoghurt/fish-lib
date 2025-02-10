use crate::config::ConfigInterface;
use crate::database::DatabaseInterface;
use crate::game::repositories::fishing_history_entry_repository::{
    FishingHistoryEntryRepository, FishingHistoryEntryRepositoryInterface,
};
use crate::game::repositories::item_repository::{ItemRepository, ItemRepositoryInterface};
use crate::game::repositories::pond_repository::{PondRepository, PondRepositoryInterface};
use crate::game::repositories::specimen_repository::{
    SpecimenRepository, SpecimenRepositoryInterface,
};
use crate::game::repositories::user_repository::{UserRepository, UserRepositoryInterface};
use crate::game::services::encounter_service::{EncounterService, EncounterServiceInterface};
use crate::game::services::fishing_history_service::{
    FishingHistoryService, FishingHistoryServiceInterface,
};
use crate::game::services::location_service::{LocationService, LocationServiceInterface};
use crate::game::services::pond_service::{PondService, PondServiceInterface};
use crate::game::services::species_service::{SpeciesService, SpeciesServiceInterface};
use crate::game::services::specimen_service::{SpecimenService, SpecimenServiceInterface};
use crate::game::services::user_service::{UserService, UserServiceInterface};
use crate::game::services::weather_service::{WeatherService, WeatherServiceInterface};
use std::sync::{Arc, RwLock};

pub trait ServiceProviderInterface: Send + Sync {
    fn config(&self) -> Arc<dyn ConfigInterface>;
    fn database(&self) -> Arc<RwLock<dyn DatabaseInterface>>;
    fn fishing_history_entry_repository(&self) -> Arc<dyn FishingHistoryEntryRepositoryInterface>;
    fn item_repository(&self) -> Arc<dyn ItemRepositoryInterface>;
    fn pond_repository(&self) -> Arc<dyn PondRepositoryInterface>;
    fn specimen_repository(&self) -> Arc<dyn SpecimenRepositoryInterface>;
    fn user_repository(&self) -> Arc<dyn UserRepositoryInterface>;
    fn encounter_service(&self) -> Arc<dyn EncounterServiceInterface>;
    fn fishing_history_service(&self) -> Arc<dyn FishingHistoryServiceInterface>;
    fn location_service(&self) -> Arc<dyn LocationServiceInterface>;
    fn pond_service(&self) -> Arc<dyn PondServiceInterface>;
    fn species_service(&self) -> Arc<dyn SpeciesServiceInterface>;
    fn specimen_service(&self) -> Arc<dyn SpecimenServiceInterface>;
    fn user_service(&self) -> Arc<dyn UserServiceInterface>;
    fn weather_service(&self) -> Arc<dyn WeatherServiceInterface>;
}

pub struct ServiceProvider {
    config: Arc<dyn ConfigInterface>,
    database: Arc<RwLock<dyn DatabaseInterface>>,
    fishing_history_entry_repository: Arc<dyn FishingHistoryEntryRepositoryInterface>,
    item_repository: Arc<dyn ItemRepositoryInterface>,
    pond_repository: Arc<dyn PondRepositoryInterface>,
    specimen_repository: Arc<dyn SpecimenRepositoryInterface>,
    user_repository: Arc<dyn UserRepositoryInterface>,
    encounter_service: Arc<dyn EncounterServiceInterface>,
    fishing_history_service: Arc<dyn FishingHistoryServiceInterface>,
    location_service: Arc<dyn LocationServiceInterface>,
    pond_service: Arc<dyn PondServiceInterface>,
    species_service: Arc<dyn SpeciesServiceInterface>,
    specimen_service: Arc<dyn SpecimenServiceInterface>,
    user_service: Arc<dyn UserServiceInterface>,
    weather_service: Arc<dyn WeatherServiceInterface>,
}

impl ServiceProvider {
    pub fn new(
        config: Arc<dyn ConfigInterface>,
        database: Arc<RwLock<dyn DatabaseInterface>>,
    ) -> Self {
        let fishing_history_entry_repository =
            Arc::new(FishingHistoryEntryRepository::new(database.clone()));
        let item_repository = Arc::new(ItemRepository::new(database.clone()));
        let pond_repository = Arc::new(PondRepository::new(database.clone()));
        let specimen_repository = Arc::new(SpecimenRepository::new(database.clone()));
        let user_repository = Arc::new(UserRepository::new(database.clone()));

        let encounter_service = Arc::new(EncounterService::new(config.clone()));
        let fishing_history_service = Arc::new(FishingHistoryService::new(
            config.clone(),
            fishing_history_entry_repository.clone(),
        ));
        let location_service = Arc::new(LocationService::new(config.clone()));
        let pond_service = Arc::new(PondService::new(pond_repository.clone()));
        let species_service = Arc::new(SpeciesService::new(config.clone()));
        let specimen_service = Arc::new(SpecimenService::new(specimen_repository.clone()));
        let user_service = Arc::new(UserService::new(
            fishing_history_entry_repository.clone(),
            user_repository.clone(),
        ));
        let weather_service = Arc::new(WeatherService::new(config.clone()));

        Self {
            config,
            database,
            fishing_history_entry_repository,
            item_repository,
            pond_repository,
            specimen_repository,
            user_repository,
            encounter_service,
            fishing_history_service,
            location_service,
            pond_service,
            species_service,
            specimen_service,
            user_service,
            weather_service,
        }
    }

    pub fn create(
        config: Arc<dyn ConfigInterface>,
        database: Arc<RwLock<dyn DatabaseInterface>>,
    ) -> Arc<dyn ServiceProviderInterface> {
        Arc::new(ServiceProvider::new(config, database))
    }
}

impl ServiceProviderInterface for ServiceProvider {
    fn config(&self) -> Arc<dyn ConfigInterface> {
        self.config.clone()
    }

    fn database(&self) -> Arc<RwLock<dyn DatabaseInterface>> {
        self.database.clone()
    }

    fn fishing_history_entry_repository(&self) -> Arc<dyn FishingHistoryEntryRepositoryInterface> {
        self.fishing_history_entry_repository.clone()
    }

    fn item_repository(&self) -> Arc<dyn ItemRepositoryInterface> {
        self.item_repository.clone()
    }

    fn pond_repository(&self) -> Arc<dyn PondRepositoryInterface> {
        self.pond_repository.clone()
    }

    fn specimen_repository(&self) -> Arc<dyn SpecimenRepositoryInterface> {
        self.specimen_repository.clone()
    }

    fn user_repository(&self) -> Arc<dyn UserRepositoryInterface> {
        self.user_repository.clone()
    }

    fn encounter_service(&self) -> Arc<dyn EncounterServiceInterface> {
        self.encounter_service.clone()
    }

    fn fishing_history_service(&self) -> Arc<dyn FishingHistoryServiceInterface> {
        self.fishing_history_service.clone()
    }

    fn location_service(&self) -> Arc<dyn LocationServiceInterface> {
        self.location_service.clone()
    }

    fn pond_service(&self) -> Arc<dyn PondServiceInterface> {
        self.pond_service.clone()
    }

    fn species_service(&self) -> Arc<dyn SpeciesServiceInterface> {
        self.species_service.clone()
    }

    fn specimen_service(&self) -> Arc<dyn SpecimenServiceInterface> {
        self.specimen_service.clone()
    }

    fn user_service(&self) -> Arc<dyn UserServiceInterface> {
        self.user_service.clone()
    }

    fn weather_service(&self) -> Arc<dyn WeatherServiceInterface> {
        self.weather_service.clone()
    }
}
