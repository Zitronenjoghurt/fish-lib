use crate::game::asset_server::access_log::AccessLog;
use crate::game::asset_server::asset_reference::{AssetReference, AssetReferenceInterface};
use crate::game::asset_server::assets::Asset;
use std::collections::HashMap;
use std::sync::{Arc, RwLock, RwLockWriteGuard};

mod access_log;
pub mod asset_reference;
pub mod assets;

pub trait AssetServerInterface: Send + Sync {
    fn put_reference(&self, reference: AssetReference);
    fn put_references(&self, references: Vec<AssetReference>);
    fn get_asset(&self, identifier: &str) -> std::io::Result<Option<Arc<Asset>>>;
    fn memory_usage(&self) -> usize;
}

#[derive(Default)]
pub struct AssetServer {
    references: RwLock<HashMap<String, Arc<AssetReference>>>,
    cached_data: RwLock<HashMap<String, Arc<Asset>>>,
    access_log: RwLock<AccessLog>,
    current_memory: RwLock<usize>,
    max_memory: usize,
}

impl AssetServer {
    pub fn new(max_memory_bytes: usize) -> Self {
        Self {
            max_memory: max_memory_bytes,
            ..Default::default()
        }
    }

    fn try_free_memory(
        &self,
        access_log_guard: &mut RwLockWriteGuard<AccessLog>,
        cached_data_guard: &mut RwLockWriteGuard<HashMap<String, Arc<Asset>>>,
    ) -> Option<usize> {
        let oldest_key = access_log_guard.pop_oldest_entry()?;

        cached_data_guard
            .remove(&oldest_key)
            .map(|asset| asset.get_size())
            .or(Some(0))
    }

    fn ensure_memory(
        &self,
        bytes: usize,
        access_log_guard: &mut RwLockWriteGuard<AccessLog>,
        cached_data_guard: &mut RwLockWriteGuard<HashMap<String, Arc<Asset>>>,
        current_memory_guard: &mut RwLockWriteGuard<usize>,
    ) -> std::io::Result<()> {
        if current_memory_guard.saturating_add(bytes) <= self.max_memory {
            return Ok(());
        }
        loop {
            let freed_memory = self.try_free_memory(access_log_guard, cached_data_guard);

            if let Some(memory) = freed_memory {
                **current_memory_guard = current_memory_guard.saturating_sub(memory);
                if current_memory_guard.saturating_add(bytes) <= self.max_memory {
                    return Ok(());
                }
            } else {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::OutOfMemory,
                    "Could not free enough memory in AssetServer",
                ));
            }
        }
    }

    fn load_asset(&self, identifier: &str) -> std::io::Result<Option<Arc<Asset>>> {
        let option_reference = self.references.read().unwrap().get(identifier).cloned();
        let Some(reference) = option_reference else {
            return Ok(None);
        };

        let prospected_memory = reference.prospect_size()?;

        let mut access_log_guard = self.access_log.write().unwrap();
        let mut cached_data_guard = self.cached_data.write().unwrap();
        let mut current_memory_guard = self.current_memory.write().unwrap();
        self.ensure_memory(
            prospected_memory,
            &mut access_log_guard,
            &mut cached_data_guard,
            &mut current_memory_guard,
        )?;

        let asset = Arc::new(Asset::load(reference)?);
        let actual_memory = asset.get_size();
        self.ensure_memory(
            actual_memory,
            &mut access_log_guard,
            &mut cached_data_guard,
            &mut current_memory_guard,
        )?;

        *current_memory_guard = current_memory_guard.saturating_add(actual_memory);
        cached_data_guard.insert(identifier.to_owned(), asset.clone());
        access_log_guard.update_entry(identifier.to_owned());

        Ok(Some(asset))
    }
}

impl AssetServerInterface for AssetServer {
    fn put_reference(&self, reference: AssetReference) {
        self.references
            .write()
            .unwrap()
            .insert(reference.get_name(), Arc::new(reference));
    }

    fn put_references(&self, references: Vec<AssetReference>) {
        let mut references_guard = self.references.write().unwrap();
        for reference in references {
            references_guard.insert(reference.get_name(), Arc::new(reference));
        }
    }

    fn get_asset(&self, identifier: &str) -> std::io::Result<Option<Arc<Asset>>> {
        let option_asset = self.cached_data.read().unwrap().get(identifier).cloned();
        if let Some(asset) = option_asset {
            Ok(Some(asset.clone()))
        } else {
            Ok(self.load_asset(identifier)?)
        }
    }

    fn memory_usage(&self) -> usize {
        *self.current_memory.read().unwrap()
    }
}
