use crate::rng::Random;
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
            mines: 10,
        }
    }
    pub fn tiles(&self) -> &Vec<Vec<Tile>> {
        &self.tiles
    }
    pub fn place(&mut self, seed: u64) {
        let mut mines_required = self.mines;
        let mut positions: Vec<(usize, usize)> = Vec::new();
        for y in 0..self.height {
            for x in 0..self.width {
                positions.push((y, x));
            }
        }
        let mut random = Random::with_seed(seed);
        loop {
            let index = random.next_int() % positions.len() as u64;
            println!("{:?}", index);
            let position: (usize, usize) = positions.remove(index as usize);
            let y = position.0;
            let x = position.1;
            &self.tiles[y][x].place_mine();
            mines_required -= 1;
            if mines_required == 0 {
                break;
            }
        }
    }
}
