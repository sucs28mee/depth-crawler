use macroquad::texture::Texture2D;

use super::{vector2::Vector2, rectangle::Rectangle, draw_data::{DrawDataCache, DrawData}, assets::Assets};


#[derive(Clone)]
pub struct Tile {
    bounds: Rectangle,
    texture_path: &'static str
}

impl Tile {
    fn draw(&self, draw_cache: &mut DrawDataCache, assets: &mut Assets, screen_position: Vector2) {
        draw_cache.add(
            DrawData::new(assets.get_texture(self.texture_path).unwrap(), self.bounds.position - screen_position)
        );
    }

    pub fn bounds(&self) -> &Rectangle {
        &self.bounds
    }
}

pub struct Tiles {
    cache: Vec<Vec<Option<Tile>>>,
    size: (usize, usize)
}

impl Tiles {
    pub fn new(size: (usize, usize)) -> Self {
        Tiles { cache: vec![vec![None; size.0]; size.1], size }
    }

    pub fn new_tile(&mut self, position: Vector2, texture_path: &'static str) {
        let index: (f32, f32) = (position.to_tile_coords() / 32.).into();
        self.cache[index.0 as usize][index.1 as usize] = Some(
            Tile {
                bounds: Rectangle { 
                    position: position.to_tile_coords(), 
                    width: 32.,
                    height: 32. 
                },
                texture_path: texture_path
            }
        );
    }

    pub fn draw_tiles(&self, draw_cache: &mut DrawDataCache, assets: &mut Assets, screen_position: Vector2) {
        for row in &self.cache {
            for tile in row {
                if tile.is_some() {
                    tile.as_ref().unwrap().draw(draw_cache, assets, screen_position);
                }
            }
        }
    }

    pub fn any<F>(&self, mut predicate: F) -> bool where F: FnMut(&Tile) -> bool{
        for row in &self.cache {
            for tile in row {
                if tile.is_some() {
                    if predicate(tile.as_ref().unwrap()) {
                        return true;
                    }
                }
            }
        }
        false
    }
}