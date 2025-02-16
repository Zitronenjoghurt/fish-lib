use crate::game::asset_server::asset_reference::{AssetReference, AssetReferenceInterface};
use crate::game::asset_server::assets::AssetType;
use std::path::PathBuf;
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq)]
pub struct ImagePngAsset {
    reference: Arc<AssetReference>,
    data: Arc<Vec<u8>>,
    size: usize,
}

impl ImagePngAsset {
    pub fn load(reference: Arc<AssetReference>) -> std::io::Result<Self> {
        let data = Arc::new(std::fs::read(reference.get_path())?);
        Ok(Self {
            reference,
            size: data.len(),
            data,
        })
    }

    pub fn get_size(&self) -> usize {
        self.size
    }

    pub fn get_data(&self) -> Arc<Vec<u8>> {
        self.data.clone()
    }
}

impl AssetReferenceInterface for ImagePngAsset {
    fn get_name(&self) -> String {
        self.reference.get_name()
    }

    fn get_path(&self) -> PathBuf {
        self.reference.get_path()
    }

    fn get_asset_type(&self) -> AssetType {
        self.reference.get_asset_type()
    }
}
