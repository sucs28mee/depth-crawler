use std::{time::Instant, ops::{Add, DerefMut, Deref}, sync::Mutex};
use super::{draw_data::DrawDataCache, flying_entity::FlyingEntity};
use macroquad::prelude::*;
use super::{assets::Assets, entity::*, player::Player, vector2::Vector2};

static mut GAME: Option<Game> = None;

pub async fn run_game(){
    unsafe {
        GAME = Some(Game::new());
        GAME.as_mut().unwrap().run().await;
    }
}

enum GameState {
    InWorld
}

pub struct Game {
    pub player: Player,
    pub entities: Entities,
    screen_position: Vector2,
    assets: Assets,
    game_state: GameState,
    fps: f32
}

impl Game {
    fn new() -> Game {
        let mut game: Game = Game {
            screen_position: Vector2::ZERO,
            player: Player::new(),
            assets: Assets::new(),
            entities: Entities::new(),
            game_state: GameState::InWorld,
            fps: 0.
        };

        game.init();
        game
    }

    fn init(&mut self) {
        self.entities.new_entity::<FlyingEntity>();
    }
    
    async fn run(&mut self) {
        let mut delta_time: f32 = 0.;
        let mut frame_start: Instant;
        loop {
            frame_start = Instant::now();
            self.update(delta_time);
            
            clear_background(BLACK);
            self.draw();
            
            next_frame().await;
            delta_time = frame_start.elapsed().as_secs_f32();
        }
    }

    fn update(&mut self, delta_time: f32) {
        self.fps = 1. / delta_time;
        match self.game_state {
            GameState::InWorld => {
                self.screen_position.lerp(self.player.hitbox.center() - Vector2::new(screen_width(), screen_height()) * 0.5, 0.05);
                unsafe {
                    let game = GAME.as_mut().unwrap();
                    self.player.update(game, delta_time);
                    self.entities.update(game, delta_time);
                }
            }
        }
    }

    fn draw(&mut self) {
        match self.game_state {
            GameState::InWorld => {
                let mut cache: DrawDataCache = DrawDataCache::new();
                self.entities.draw(&mut cache, &mut self.assets, self.screen_position);
                self.player.draw(&mut cache, &mut self.assets, self.screen_position);
                cache.draw_cache();
            }
        }
        
        draw_text(self.fps.to_string().add(" FPS").as_str(), 0., 14., 16., YELLOW);
    }
}