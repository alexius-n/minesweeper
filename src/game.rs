use crate::board::{self, Board};
use crate::game::GameState::Lost;
use crate::rng::Random;
use crate::tile::TileState::{self, Flagged, Hidden};
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
        let mut new = Board::create(20, 20);
        let mut random = Random::new();
        let seed = random.next_int();
        new.place(seed);
        new.adjacent();
        Self {
            board: new,
            flagged: (0),
            game_state: GameState::NotStarted,
            seed: seed,
        }
    }
    pub fn board(&self) -> &Board {
        &self.board
    }
    pub fn flag(&mut self, cordirnates: (usize, usize)) {
        if self.board().get_tiles(cordirnates).tile_state() == &Flagged {
            self.board.edit_state(cordirnates, TileState::Hidden);
        } else {
            self.board.edit_state(cordirnates, TileState::Flagged);
        }
    }
    pub fn dig(&mut self, cordinates: (usize, usize)) {
        if self.board.get_tiles(cordinates).tile_state() == &Hidden {
            self.board.edit_state(cordinates, TileState::Dug);
            if self.board.get_tiles(cordinates).is_mine() {
                self.game_state = Lost;
                println!("U lost loser");
            } else {
                println!("u dig");
            }
        }
    }
}
