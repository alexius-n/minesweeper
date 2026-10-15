#[derive(PartialEq, Debug, Default, Clone)]
pub enum TileState {
    #[default]
    Hidden,
    Flagged,
    Dug,
}

#[derive(Default, Debug, Clone)]
pub struct Tile {
    is_mine: bool,
    tile_state: TileState,
    mines_adjacent: u8,
}

impl Tile {
    pub fn new() -> Self {
        let is_mine = false;
        let tile_state = TileState::Dug;
        let mines_adjacent = 0;
        Self {
            is_mine,
            tile_state,
            mines_adjacent,
        }
    }
    //getters
    pub fn is_mine(&self) -> bool {
        self.is_mine
    }
    pub fn tile_state(&self) -> &TileState {
        &self.tile_state
    }
    pub fn mines_adjacent(&self) -> u8 {
        self.mines_adjacent
    }
    //setters
    pub fn place_mine(&mut self) {
        self.is_mine = true;
    }
    pub fn update_adjacent(&mut self) {
        self.mines_adjacent += 1;
    }
    pub fn update_state(&mut self, state: TileState) {
        self.tile_state = state;
    }
}
