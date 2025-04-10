const MAP_SIZE: (u16, u16) = (10, 10);

pub struct Map {
    pub cells: Vec<Vec<bool>>,
    pub size: (u16, u16),
}

impl Map {
    pub fn new() -> Self {
        let cells = vec![vec![false; MAP_SIZE.0 as usize]; MAP_SIZE.1 as usize];
        Self {
            cells,
            size: MAP_SIZE,
        }
    }

    fn generate_basic_map(&mut self) {
        for y in 0..self.size.1 {
            for x in 0..self.size.0 {
                self.cells[y as usize][x as usize] = x == 0
                    || y == 0
                    || x == self.size.0 - 1
                    || y == self.size.1 - 1
                    || (x % 10 == 0 && y % 5 == 0);
            }
        }
    }
}

impl Default for Map {
    fn default() -> Self {
        let mut map = Self::new();
        map.generate_basic_map();
        map
    }
}
