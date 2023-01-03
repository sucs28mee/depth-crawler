use macroquad::prelude::{is_key_down, KeyCode, Rect};
use super::game::Game;
use super::tile::{Tiles, self};
use super::{entity::*, assets::Assets, vector2::Vector2, rectangle::Rectangle};
use super::draw_data::*;

pub enum Direction {
    Right,
    Left
}

pub struct Player {
    pub hitbox: Rectangle,
    pub velocity: Vector2,
    pub direction: Direction,
    pub jump_timer: i32,
    frame: f32
}


impl Player {
    const MAX_JUMP_FRAMES: i32 = 35;

    pub fn translate(&mut self, translation: Vector2) {
        self.hitbox.position += translation;
    }
}

impl Entity for Player {
    fn new() -> Self where Self: Sized {
        Player { 
            hitbox: Rectangle::new(0., 0., 24., 30.),
            velocity: Vector2::ZERO,
            direction: Direction::Right,
            jump_timer: 0,
            frame: 0.
        }
    }
    
    fn update(&mut self, game: &mut Game, delta_time: f32) {
        let move_speed = 40. * delta_time;
        let mut move_input: bool = false;
        if is_key_down(KeyCode::D) {
            self.velocity += Vector2::UNIT_X * move_speed;

            self.direction = Direction::Right;
            move_input = true;
        }
        if is_key_down(KeyCode::A) {
            self.velocity -= Vector2::UNIT_X * move_speed;

            self.direction = Direction::Left;
            move_input = true;
        }
        
        let mut can_jump = false;
        // Apply gravity and check for collision.
        self.velocity.y += 0.4;
        game.tiles.any(|tile| {
                if self.hitbox.translated(Vector2::new(0., self.velocity.y)).intersects(tile.bounds()) {
                    if self.velocity.y > 0. {
                        can_jump = true; // Tile under player.
                    }
                    self.velocity.y = 0.;
                }

                if self.hitbox.translated(Vector2::new(self.velocity.x, 0.)).intersects(tile.bounds()) {
                    self.velocity.x = 0.;
                }

                if self.velocity == Vector2::ZERO {
                    return true;
                }
                false
            }
        );

        // In case there is a tile under player, the player can jump so check for input.
        if is_key_down(KeyCode::Space) {
            if can_jump {
                self.jump_timer = Self::MAX_JUMP_FRAMES;
            }

            if self.jump_timer > 0  {
                self.velocity.y -= 1. * (self.jump_timer as f32) / (Self::MAX_JUMP_FRAMES as f32);
                self.jump_timer -= 1;
            }
        }
        else {
            self.jump_timer = 0;
        }

        if self.velocity.y == 0. {
            self.velocity.x *= 0.87; // Friction!
            if move_input {
                self.frame = if self.frame >= 4. { 0. } else { self.frame + 0.15 };
            }
            else {
                self.frame = 0.;
            }
        }
        else if self.velocity.y < 0. {
            self.frame = 1.;
        }
        else {
            self.frame = 2.;
        }
        
        self.velocity.x *= 0.95; // Air resistance and stuff or something idk
        self.translate(self.velocity);
    }

    fn draw(&self, cache: &mut DrawDataCache, assets: &mut Assets, screen_position: Vector2) {
        //self.hitbox.draw(-screen_position);
        cache.add(DrawData::new_custom(
            assets.get_texture("butter.png").unwrap(), 
            self.hitbox.center() - screen_position,
            |data| {
                data.draw_layer = DrawLayer::OverEntities;
                data.origin = Vector2::new(8., 8.);
                data.rotation = (self.velocity.x * 0.08).clamp(-0.25, 0.25);
                data.flip_sprite = match self.direction {
                    Direction::Right => None,
                    Direction::Left => Some(FlipSprite::FlipHorizontal),
                };
                data.source = Some(Rect::new(0., 16. * self.frame.floor(), 16., 16.));
            }
            )
        );
    }
}