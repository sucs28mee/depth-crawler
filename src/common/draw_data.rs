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
    cache: Vec<DrawData>,
    default_scale: f32
}

impl DrawDataCache {
    pub fn new() -> Self {
        DrawDataCache { cache: Vec::new(), default_scale: 2. }
    }

    pub fn add(&mut self, draw_data: DrawData) {
        self.cache.push(draw_data);
    }

    /// Draws whats in the cache and clears it.
    pub fn draw(&mut self) {
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

            let draw_pos = draw_data.position - draw_data.origin * 2.;
            let mut frame_size: Vec2 = Vec2::new(draw_data.size.x, draw_data.size.y);
            if draw_data.source.is_some() {
                let source_ref = draw_data.source.as_ref().unwrap();
                frame_size *= source_ref.size();
            }
            else {
                frame_size *= Vec2::new(draw_data.texture.width(), draw_data.texture.height());
            }

            draw_texture_ex(
                draw_data.texture, 
                draw_pos.x,
                draw_pos.y,
                draw_data.color,
                DrawTextureParams {
                    dest_size: Some(self.default_scale * frame_size),
                    source: draw_data.source,
                    rotation: draw_data.rotation,
                    flip_x,
                    flip_y,
                    pivot: Some((draw_data.position).as_vec2())
                }
            );
        }

        self.cache.clear();
    }
}