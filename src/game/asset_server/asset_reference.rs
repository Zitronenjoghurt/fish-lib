use crate::game::asset_server::assets::AssetType;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

pub trait AssetReferenceInterface: Send + Sync {
    fn get_name(&self) -> String;
    fn get_path(&self) -> PathBuf;
    fn get_asset_type(&self) -> AssetType;
    fn prospect_size(&self) -> std::io::Result<usize> {
        let metadata = std::fs::metadata(self.get_path())?;
        Ok(metadata.len() as usize)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AssetReference {
    name: String,
    path: PathBuf,
    asset_type: AssetType,
}

impl AssetReference {
    pub fn new_png(name: &str, path: &Path) -> Self {
        Self {
            name: name.to_owned(),
            path: PathBuf::from(path),
            asset_type: AssetType::ImagePng,
        }
    }
}

impl AssetReferenceInterface for AssetReference {
    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_path(&self) -> PathBuf {
        self.path.clone()
    }

    fn get_asset_type(&self) -> AssetType {
        self.asset_type
    }
}
