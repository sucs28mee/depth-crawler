mod common;
use common::game::Game;

#[macroquad::main("Depth Crawler")]
async fn main() {
    Game::new().run().await;
}
