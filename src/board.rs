use crate::tile::{self, Tile};
#[derive(Debug)]
pub struct Board {
    tiles: Vec<Vec<Tile>>,
    width: usize,
    height: usize,
    mines: usize,
}
impl Board {
    pub fn create(width: usize, height: usize) -> Self {
        let tiles = vec![vec![Tile::new(); width]; height];
        Self {
            tiles,
            width,
            height,
            mines: 40,
        }
    }
    pub fn tiles(&self) -> &Vec<Vec<Tile>> {
        &self.tiles
    }
    pub fn place(seed: u64, mines: usize) {}
}
