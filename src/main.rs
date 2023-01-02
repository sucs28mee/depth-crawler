mod common;
use common::game;

#[macroquad::main("Depth Crawler")]
async fn main() {
    game::run_game().await;
}
