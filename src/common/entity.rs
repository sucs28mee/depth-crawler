use macroquad::{prelude::*};
use super::{assets::Assets, vector2::Vector2};
use super::draw_data::DrawDataCache;

pub trait Entity {
    fn new() -> Self where Self: Sized;
    fn update(&mut self, delta_time: f32);
    fn draw(&self, draw_cache: &mut DrawDataCache, assets: &mut Assets, screen_position: Vector2);
}

pub struct Entities {
    cache: Vec<Box<dyn Entity>>
}

impl Entities {
    pub fn new() -> Self {
        Self { cache: Vec::new() }
    }

    pub fn new_entity<T>(&mut self) where T : Entity + 'static {
        self.cache.push(Box::new(T::new()));
    }

    pub fn update(&mut self, delta_time: f32) {
        for entity in &mut self.cache {
            entity.update(delta_time);
        }
    }

    pub fn draw(&mut self, cache: &mut DrawDataCache, assets: &mut Assets, screen_position: Vector2) {
        for entity in &mut self.cache {
            entity.draw(cache, assets, screen_position);
        }
    }

    pub fn as_vec(&mut self) -> &mut Vec<Box<dyn Entity>> {
        &mut self.cache
    }
}