use crate::game::Game;
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
