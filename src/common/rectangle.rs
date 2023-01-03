#![macro_use]
use macroquad::{shapes::draw_rectangle, prelude::{RED, Rect}};

use super::vector2::Vector2;

#[derive(Clone, Copy, Debug)]
pub struct Rectangle {
    pub position: Vector2,
    pub width: f32,
    pub height: f32
}

impl Rectangle {
    #[inline]
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            position: Vector2::new(x, y),
            width,
            height
        }
    }

    pub fn translated(&self, translation: Vector2) -> Rectangle {
        let mut copy = self.clone();
        copy.position += translation;
        copy
    }

    #[inline]
    pub fn bottom(&self) -> f32 {
        self.position.y + self.height
    }
    
    #[inline]
    pub fn right(&self) -> f32 {
        self.position.x + self.width
    }

    #[inline]
    pub fn bottom_right(&self) -> Vector2 {
        self.position + self.size()
    }

    #[inline]
    pub fn size(&self) -> Vector2 {
        Vector2::new(self.width, self.height)
    }

    #[inline]
    pub fn center(&self) -> Vector2 {
        self.position + self.size() * 0.5
    }

    #[inline]
    pub fn contains(&self, point: Vector2) -> bool {
        point.x > self.position.x && point.y > self.position.y &&
            point.x < self.bottom_right().x && point.y < self.bottom_right().y
    }

    #[inline]
    pub fn as_rect(&self) -> Rect {
        Rect { x: self.position.y, y: self.position.y, w: self.width, h: self.height }
    }

    pub fn intersects(&self, other: &Rectangle) -> bool {
        if other.position.x < self.right() && self.position.x < other.right() && other.position.y < self.bottom() {
            return self.position.y < other.bottom();
        }
        false
    }

    pub fn draw(&self, offset: Vector2) {
        let pos = self.position + offset;
        draw_rectangle(pos.x, pos.y, self.width, self.height, RED);
    }
}