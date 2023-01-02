use macroquad::prelude::{is_key_down, KeyCode, mouse_position_local};
use macroquad::window::{screen_width, screen_height};

use super::extensions::Vec2Extensions;
use super::game::Game;
use super::{entity::*, assets::Assets, vector2::Vector2, rectangle::Rectangle};
use super::draw_data::*;
pub struct Player {
    pub hitbox: Rectangle
}

impl Player {
    pub fn translate(&mut self, translation: Vector2) {
        self.hitbox.position += translation;
    }
}

impl Entity for Player {
    fn new() -> Self where Self: Sized {
        Player { hitbox: Rectangle::new(0., 0., 80., 80.) }
    }
    
    fn update(&mut self, game: &mut Game, delta_time: f32) {
        let move_speed = 150. * delta_time;
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

    fn draw(&self, cache: &mut DrawDataCache, assets: &mut Assets, screen_position: Vector2) {
        let draw_pos = self.hitbox.center() - screen_position;
        let screen_size = Vector2::new(screen_width(), screen_height()) * 0.5;
        let mouse_pos = mouse_position_local().as_vector2() * screen_size + screen_size;
        let dir_to_mouse = draw_pos.dir_to(mouse_pos);
        cache.add(DrawData::new_custom(
            assets.get_texture("hekatomb.png").unwrap(), 
            draw_pos,
            |data| {
                data.draw_layer = DrawLayer::OverEntities;
                data.rotation = dir_to_mouse.to_rotation();
                data.origin = Vector2::new(data.texture.width(), data.texture.height()) * 0.5;
            }
            )
        );
    }
}