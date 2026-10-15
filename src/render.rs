use crate::board::Board;
use crate::game::Game;
use crate::tile::{self, TileState};
use macroquad::prelude::*;
pub async fn load_textures() -> Vec<Texture2D> {
    let zero: Texture2D = load_texture("assets/0.png").await.unwrap();
    let one: Texture2D = load_texture("assets/1.png").await.unwrap();
    let two: Texture2D = load_texture("assets/2.png").await.unwrap();
    let three: Texture2D = load_texture("assets/3.png").await.unwrap();
    let four: Texture2D = load_texture("assets/4.png").await.unwrap();
    let five: Texture2D = load_texture("assets/5.png").await.unwrap();
    let six: Texture2D = load_texture("assets/6.png").await.unwrap();
    let seven: Texture2D = load_texture("assets/7.png").await.unwrap();
    let eight: Texture2D = load_texture("assets/8.png").await.unwrap();
    let flag: Texture2D = load_texture("assets/flag.png").await.unwrap();
    let bomb: Texture2D = load_texture("assets/bomb.png").await.unwrap();
    let hidden: Texture2D = load_texture("assets/hidden.png").await.unwrap();

    zero.set_filter(FilterMode::Nearest);
    one.set_filter(FilterMode::Nearest);
    two.set_filter(FilterMode::Nearest);
    three.set_filter(FilterMode::Nearest);
    four.set_filter(FilterMode::Nearest);
    five.set_filter(FilterMode::Nearest);
    six.set_filter(FilterMode::Nearest);
    seven.set_filter(FilterMode::Nearest);
    eight.set_filter(FilterMode::Nearest);
    flag.set_filter(FilterMode::Nearest);
    bomb.set_filter(FilterMode::Nearest);
    hidden.set_filter(FilterMode::Nearest);

    vec![
        zero, one, two, three, four, five, six, seven, eight, hidden, flag, bomb,
    ]
}
pub async fn render(game: Game) {
    let textures = load_textures().await;
    loop {
        clear_background(GRAY);
        let mut x = 0.0;
        let mut y = 0.0;
        for row in game.board().tiles() {
            for tile in row {
                match tile.tile_state() {
                    TileState::Dug => match tile.mines_adjacent() {
                        0 => {
                            if tile.is_mine() == true {
                                draw_texture(&textures[11], (x * 32.0), (y * 32.0), WHITE);
                            } else {
                                draw_texture(&textures[0], (x * 32.0), (y * 32.0), WHITE);
                            }
                        }
                        1 => draw_texture(&textures[1], (x * 32.0), (y * 32.0), WHITE),
                        2 => draw_texture(&textures[2], (x * 32.0), (y * 32.0), WHITE),
                        3 => draw_texture(&textures[3], (x * 32.0), (y * 32.0), WHITE),
                        4 => draw_texture(&textures[4], (x * 32.0), (y * 32.0), WHITE),
                        5 => draw_texture(&textures[5], (x * 32.0), (y * 32.0), WHITE),
                        6 => draw_texture(&textures[6], (x * 32.0), (y * 32.0), WHITE),
                        7 => draw_texture(&textures[7], (x * 32.0), (y * 32.0), WHITE),
                        8 => draw_texture(&textures[8], (x * 32.0), (y * 32.0), WHITE),
                        9_u8..=u8::MAX => panic!("Invalid Number"),
                    },
                    TileState::Flagged => {
                        draw_texture(&textures[10], (x * 32.0), (y * 32.0), WHITE)
                    }
                    TileState::Hidden => draw_texture(&textures[9], (x * 32.0), (y * 32.0), WHITE),
                }

                x += 1.0;
            }
            x = 0.0;
            y += 1.0;
        }
        next_frame().await;
    }
}
