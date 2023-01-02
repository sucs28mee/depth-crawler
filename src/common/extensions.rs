use macroquad::prelude::Vec2;

use super::vector2::Vector2;


pub trait Vec2Extensions {
    fn as_vector2(&self) -> Vector2;
}

impl Vec2Extensions for Vec2 {
    fn as_vector2(&self) -> Vector2 {
        Vector2 { x: self.x, y: self.y }
    }
}