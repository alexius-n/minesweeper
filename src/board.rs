use crate::rng::Random;
use crate::tile::TileState::Dug;
use crate::tile::{self, Tile, TileState};
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
            mines: 24,
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
            let random = random.next_int();
            assert!(!positions.is_empty(), "No positions left!");
            let index = random % positions.len() as u64;
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
    pub fn adjacent(&mut self) {
        let mut positions: Vec<(usize, usize)> = Vec::new();
        for (y, row) in self.tiles.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                if tile.is_mine() {
                    let nx: isize = x as isize;
                    let ny: isize = y as isize;
                    let calculations: [isize; 3] = [-1, 0, 1];
                    for dy in calculations {
                        for dx in calculations {
                            let fx = nx + dx;
                            let fy = ny + dy;
                            if 0 <= fy && fy < self.height as isize {
                                if 0 <= fx && fx < self.width as isize {
                                    if self.tiles[fy as usize][fx as usize].is_mine() == false {
                                        println!("+1 at {},{}", fx, fy);
                                        positions.push((fy as usize, fx as usize));
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        for (y, x) in positions {
            self.tiles[y][x].update_adjacent();
        }
    }
    pub fn edit_state(&mut self, cordirnates: (usize, usize), state: TileState) {
        self.tiles[cordirnates.1][cordirnates.0].update_state(state);
    }
    pub fn get_tiles(&self, cordirnates: (usize, usize)) -> &Tile {
        &self.tiles[cordirnates.1][cordirnates.0]
    }
    pub fn chord(&mut self) {
        let mut positions: Vec<(usize, usize)> = Vec::new();
        for (y, row) in self.tiles.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                if tile.mines_adjacent() == 0 {
                    let nx: isize = x as isize;
                    let ny: isize = y as isize;
                    let calculations: [isize; 3] = [-1, 0, 1];
                    for dy in calculations {
                        for dx in calculations {
                            let fx = nx + dx;
                            let fy = ny + dy;
                            if 0 <= fy && fy < self.height as isize {
                                if 0 <= fx && fx < self.width as isize {
                                    if self.tiles[fy as usize][fx as usize].is_mine() == false {
                                        println!("+1 at {},{}", fx, fy);
                                        positions.push((fy as usize, fx as usize));
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        for (y, x) in positions {
            self.tiles[y][x].update_state(Dug);
        }
    }
}
