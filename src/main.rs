mod common;
use common::game::Game;

#[macroquad::main("terraria-but-scuffed")]
async fn main() {
    Game::new().run().await;
}
