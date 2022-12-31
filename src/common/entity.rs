use macroquad::{prelude::*, texture::{Texture2D, draw_texture_ex}};
use super::{assets::Assets, vector2::Vector2};
use super::draw_data::DrawDataCache;

pub trait Entity {
    fn update(&mut self, delta_time: f32);
    fn draw(&self, draw_cache: &mut DrawDataCache, assets: &mut Assets, screen_position: Vector2);
}