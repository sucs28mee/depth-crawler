use std::slice::Iter;

use macroquad::prelude::*;

use super::vector2::Vector2;

#[derive(Debug, Clone, Copy)]
pub enum FlipSprite {
    FlipHorizontal,
    FlipVertical
}

#[derive(Debug, Clone, Copy)]
pub enum DrawLayer {
    Entities,
    OverEntities
}

#[derive(Debug)]
pub struct DrawData {
    pub draw_layer: DrawLayer,
    pub texture: Texture2D,
    pub position: Vector2,
    pub rotation: f32,
    pub color: Color,
    pub origin: Vector2,
    pub size: Vector2,
    pub source: Option<Rect>,
    pub flip_sprite: Option<FlipSprite>
}

impl DrawData {
    pub fn new(texture: Texture2D, position: Vector2) -> DrawData {
        DrawData { draw_layer: DrawLayer::Entities, texture, position, rotation: 0., color: WHITE, origin: Vector2::ZERO, size: Vector2::ONE, source: None, flip_sprite: None }
    }

    pub fn new_custom<F>(texture: Texture2D, position: Vector2, func: F) -> DrawData where F: FnOnce(&mut Self) {
        let mut draw_data: DrawData = DrawData { 
            draw_layer: DrawLayer::Entities, 
            texture, 
            position, 
            rotation: 0., 
            color: WHITE, 
            origin: Vector2::ZERO, 
            size: Vector2::ONE, 
            source: None, 
            flip_sprite: None 
        };
        func(&mut draw_data);
        draw_data
    }
}

pub struct DrawDataCache {
    cache: Vec<DrawData>
}

impl DrawDataCache {
    pub fn new() -> Self {
        DrawDataCache { cache: Vec::new() }
    }

    pub fn add(&mut self, draw_data: DrawData) {
        self.cache.push(draw_data);
    }

    pub fn draw_cache(&mut self) {
        self.cache.sort_by(|a, b| (a.draw_layer as u8).cmp(&(b.draw_layer as u8)));

        let cache_iter: Iter<DrawData> = self.cache.iter();
        for draw_data in cache_iter {
            let mut flip_x = false;
            let mut flip_y = false;

            if draw_data.flip_sprite.is_some() {
                match draw_data.flip_sprite.unwrap() {
                    FlipSprite::FlipHorizontal => flip_x = true,
                    FlipSprite::FlipVertical => flip_y = true
                }
            }

            draw_texture_ex(
                draw_data.texture, 
                draw_data.position.x,
                draw_data.position.y,
                draw_data.color,
                DrawTextureParams { 
                    dest_size: Some(Vec2::new(draw_data.size.x * draw_data.texture.width(), draw_data.size.y * draw_data.texture.height())),
                    source: draw_data.source, 
                    rotation: draw_data.rotation, 
                    flip_x, 
                    flip_y, 
                    pivot: Some((draw_data.origin + draw_data.position).as_vec2()) 
                }
            );
        }
    }
}