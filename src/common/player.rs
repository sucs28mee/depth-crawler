use super::{entity::*, assets::Assets, vector2::Vector2, rectangle::Rectangle};
use super::draw_data::*;
pub struct Player {
    pub hitbox: Rectangle
}

impl Player {
    pub fn new() -> Player {
        Player { hitbox: Rectangle::new(10., 10., 10., 10.) }
    }
}

impl Entity for Player {
    fn update(&mut self, delta_time: f32) {
        //self.hitbox.position += Vector2::ONE * 10. * delta_time;
    }

    fn draw(&self, cache: &mut DrawDataCache, assets: &mut Assets, screen_position: Vector2) {
        cache.add(DrawData::new(assets.get_texture("hekatomb.png").unwrap(), self.hitbox.position - screen_position));
    }
}