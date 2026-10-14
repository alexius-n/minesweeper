use crate::board::{self, Board};
use crate::rng::Random;
#[derive(PartialEq, Debug, Default)]
enum GameState {
    #[default]
    NotStarted,
    Started,
    Lost,
    Won,
}
#[derive(Debug)]
pub struct Game {
    board: Board,
    flagged: usize,
    game_state: GameState,
    pub seed: u64,
}
impl Game {
    pub fn new() -> Self {
        let new = Board::create(40, 40);
        let mut random = Random::new();
        let seed = random.next_int();
        Self {
            board: new,
            flagged: (0),
            game_state: GameState::NotStarted,
            seed: seed,
        }
    }
}
