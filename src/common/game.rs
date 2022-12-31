use std::{time::Instant, ops::Add};
use super::draw_data::DrawDataCache;
use macroquad::prelude::*;
use super::{assets::Assets, entity::*, player::Player, vector2::Vector2};
enum GameState {
    InWorld
}

pub struct Game {
    screen_position: Vector2,
    pub player: Player,
    assets: Assets,
    entities: Vec<Box<dyn Entity>>,
    game_state: GameState,
    fps: f32
}

impl Game {
    pub fn new() -> Game {
        let mut game: Game = Game {
            screen_position: Vector2::ZERO,
            player: Player::new(),
            assets: Assets::new(),
            entities: Vec::new(),
            game_state: GameState::InWorld,
            fps: 0.
        };
        game.init();
        
        return game;
    }

    pub fn init(&mut self) {
        
    }
    
    pub async fn run(&mut self) {
        let mut frame_start = Instant::now();
        loop {
            self.update(frame_start.elapsed().as_secs_f32());
            frame_start = Instant::now();
            
            clear_background(BLACK);
            self.draw();
            
            next_frame().await;
        }
    }

    fn update(&mut self, delta_time: f32) {
        self.fps = 1f32 / delta_time;
        match self.game_state {
            GameState::InWorld => {
                self.screen_position.lerp(self.player.hitbox.center() - Vector2::new(screen_width(), screen_height()), 0.15);

                self.player.update(delta_time);
                for entity in &mut self.entities {
                    entity.update(delta_time);
                }
            }
        }
        
    }

    fn draw(&mut self) {
        match self.game_state {
            GameState::InWorld => {
                let mut cache: DrawDataCache = DrawDataCache::new();
                for entity in &mut self.entities {
                    entity.draw(&mut cache, &mut self.assets, self.screen_position);
                }
                
                self.player.draw(&mut cache, &mut self.assets, self.screen_position);

                cache.draw_cache();
            }
        }
        
        draw_text(self.fps.to_string().add(" fps").as_str(), 0., 14., 16., YELLOW);
    }
}