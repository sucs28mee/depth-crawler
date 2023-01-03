use rust_embed::RustEmbed;
use macroquad::prelude::*;
use std::{collections::HashMap};

#[derive(Debug)]
pub enum AssetError {
    AssetNotFound
}

pub struct Assets {
    asset_cache: HashMap<&'static str, Texture2D>
}

impl Assets {
    pub fn new() -> Assets {
        Assets { asset_cache: HashMap::new() }
    }

    /// Gets the texture with specified path from the assets folder.
    pub fn get_texture(&mut self, path: &'static str) -> Result<Texture2D, AssetError> {
        if !self.asset_cache.contains_key(path) {
            let file = AssetLoader::get(path).ok_or(AssetError::AssetNotFound)?;
            let texture = Texture2D::from_file_with_format(&file.data, None);
            texture.set_filter(FilterMode::Nearest);
            self.asset_cache.insert(path, texture);
        }

        Ok(self.asset_cache[path])
    }
}

#[derive(RustEmbed)]
#[folder = "assets"]
struct AssetLoader;