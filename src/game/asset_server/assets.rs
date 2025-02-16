use crate::game::asset_server::asset_reference::{AssetReference, AssetReferenceInterface};
use crate::game::asset_server::assets::image_png::ImagePngAsset;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::Arc;

mod image_png;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Serialize, Deserialize)]
pub enum AssetType {
    ImagePng,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Asset {
    ImagePng(ImagePngAsset),
}

impl Asset {
    pub fn load(reference: Arc<AssetReference>) -> std::io::Result<Self> {
        match reference.get_asset_type() {
            AssetType::ImagePng => Ok(Self::ImagePng(ImagePngAsset::load(reference)?)),
        }
    }

    pub fn get_size(&self) -> usize {
        match self {
            Self::ImagePng(asset) => asset.get_size(),
        }
    }

    pub fn as_png(&self) -> Option<&ImagePngAsset> {
        match self {
            Self::ImagePng(asset) => Some(asset),
            _ => None,
        }
    }
}

impl AssetReferenceInterface for Asset {
    fn get_name(&self) -> String {
        match self {
            Self::ImagePng(asset) => asset.get_name(),
        }
    }

    fn get_path(&self) -> PathBuf {
        match self {
            Self::ImagePng(asset) => asset.get_path(),
        }
    }

    fn get_asset_type(&self) -> AssetType {
        match self {
            Self::ImagePng(asset) => asset.get_asset_type(),
        }
    }
}
