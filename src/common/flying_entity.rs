use super::{entity::Entity, draw_data::DrawData, rectangle::Rectangle};


pub struct FlyingEntity {
    hitbox: Rectangle
}

impl Entity for FlyingEntity {
    fn new() -> Self where Self: Sized {
        FlyingEntity { hitbox: Rectangle::new(0., 0., 10., 10.) }
    }

    fn update(&mut self, game: &mut super::game::Game, delta_time: f32) {
        
    }

    fn draw(&self, draw_cache: &mut super::draw_data::DrawDataCache, assets: &mut super::assets::Assets, screen_position: super::vector2::Vector2) {
        draw_cache.add(
            DrawData::new(assets.get_texture("hekatomb.png").unwrap(), self.hitbox.position - screen_position)
        )
    }
}