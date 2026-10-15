mod board;
mod game;
mod render;
mod rng;
mod tile;
use crate::game::Game;
use crate::render::render;

#[macroquad::main("Minesweeper")]
async fn main() {
    let game = Game::new();
    render(game).await;
}
