use bevy::asset::{AssetLoader, LoadContext, io::Reader};
use bevy::prelude::*;
use thiserror::Error;

#[derive(Asset, TypePath, Debug)]
pub struct RonAsset;

#[derive(Default)]
pub struct RonLoader;

#[derive(Debug, Error)]
pub enum RonLoaderError {
    #[error("Could not load asset: {0}")]
    Io(#[from] std::io::Error),
}

impl AssetLoader for RonLoader {
    type Asset = RonAsset;
    type Settings = ();
    type Error = RonLoaderError;

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &(),
        _load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut _bytes = Vec::new();
        reader.read_to_end(&mut _bytes).await?;
        Ok(RonAsset)
    }

    fn extensions(&self) -> &[&str] {
        &["ron"]
    }
}
