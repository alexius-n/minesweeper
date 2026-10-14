mod board;
mod game;
mod render;
mod rng;
mod tile;
use crate::game::Game;
use crate::render::load_textures;
use crate::rng::Random;
use macroquad::prelude::*;

#[macroquad::main("Minesweeper")]
async fn main() {
    /*for i in 0..10 {
        let mut random = Random::new();
        let x: u64 = random.next_int();
        println!("{}", x);
    }*/
    let game = Game::new();
    let textures = load_textures().await;
    println!("New Game With Seed: {}", game.seed);

    loop {
        clear_background(GRAY);
        for i in 0..10 {
            let y: f32 = i as f32;
            for i in 0..10 {
                let mut x: f32 = i as f32;
                draw_texture(&textures[10], (x * 32.0), (y * 32.0), WHITE);
            }
        }

        next_frame().await;
    }
}
