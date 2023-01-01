use macroquad::prelude::{is_key_down, KeyCode};

use super::{entity::*, assets::Assets, vector2::Vector2, rectangle::Rectangle};
use super::draw_data::*;
pub struct Player {
    pub hitbox: Rectangle
}

impl Player {
    pub fn translate(&mut self, by: Vector2) {
        self.hitbox.position += by;
    }

    pub fn new() -> Self where Self: Sized {
        Player { hitbox: Rectangle::new(0., 0., 50., 50.) }
    }

    pub fn update(&mut self, delta_time: f32) {
        let move_speed = 50. * delta_time;
        if is_key_down(KeyCode::D) {
            self.translate(Vector2::UNIT_X * move_speed);
        }
        if is_key_down(KeyCode::A) {
            self.translate(-Vector2::UNIT_X * move_speed);
        }
        if is_key_down(KeyCode::W) {
            self.translate(-Vector2::UNIT_Y * move_speed);
        }
        if is_key_down(KeyCode::S) {
            self.translate(Vector2::UNIT_Y * move_speed);
        }
    }

    pub fn draw(&self, cache: &mut DrawDataCache, assets: &mut Assets, screen_position: Vector2) {
        cache.add(DrawData::new_custom(
            assets.get_texture("hekatomb.png").unwrap(), 
            self.hitbox.position - screen_position,
            |data| data.draw_layer = DrawLayer::OverEntities
            )
        );
    }
}