use std::time::SystemTime;
#[derive(Clone, Debug)]
pub struct Random {
    state: u64,
}

impl Random {
    pub fn new() -> Self {
        let seed = Random::clock_seed();
        Random { state: seed }
    }
    pub fn clock_seed() -> u64 {
        match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
            Ok(n) => n.as_nanos() as u64,
            Err(_) => panic!("SystemTime before UNIX EPOCH!"),
        }
    }
    pub fn with_seed(seed: u64) -> Self {
        Random { state: seed }
    }
    pub fn next_int(&mut self) -> u64 {
        let mut z = self.state.wrapping_add(0x9e3779b97f4a7c15);
        self.state = z;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
        z ^ (z >> 31)
    }
}
