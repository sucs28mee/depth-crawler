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

    #[inline]
    pub fn bottom_left(&self) -> Vector2 {
        self.position + self.size()
    }

    #[inline]
    pub fn size(&self) -> Vector2 {
        Vector2::new(self.width, self.height)
    }

    #[inline]
    pub fn center(&self) -> Vector2 {
        self.position + self.size() * 0.5f32
    }

    #[inline]
    pub fn contains(&self, point: Vector2) -> bool {
        point.x > self.position.x && point.y > self.position.y && 
            point.x < self.bottom_left().x && point.y < self.bottom_left().y
    }
}