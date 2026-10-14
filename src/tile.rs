#[derive(PartialEq, Debug, Default, Clone)]
enum TileState {
    #[default]
    Hidden,
    Flagged,
    Dug,
}

#[derive(Default, Debug, Clone)]
pub struct Tile {
    is_bomb: bool,
    tile_state: TileState,
    bombs_adjacent: u8,
}

impl Tile {
    pub fn new() -> Self {
        let is_bomb = false;
        let tile_state = TileState::Hidden;
        let bombs_adjacent = 0;
        Self {
            is_bomb,
            tile_state,
            bombs_adjacent,
        }
    }
}
