#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Direction {
    Forward,
    Backward,
}

#[derive(Clone, Copy, Debug)]
pub enum Tile {
    Empty,
    Variable(u8),
    Const(u8),
}

#[derive(Clone)]
pub struct Sudoku {
    pub tiles: Vec<Tile>,
    pub active_indx: usize,
    pub direction: Direction,
    pub running: bool,
    pub step_count: u64,
    pub steps_per_frame: f64,
    pub solve_instantly: bool,
    substeps: u64,
}

impl Default for Sudoku {
    fn default() -> Self {
        Sudoku {
            tiles: vec![Tile::Empty; 81],
            active_indx: 0,
            direction: Direction::Forward,
            running: false,
            step_count: 0,
            steps_per_frame: 1.0,
            solve_instantly: false,
            substeps: 0,
        }
    }
}

impl Sudoku {
    pub fn try_insert(&mut self, indx: usize, tile: Tile) {
        let tmp = self.tiles[indx];
        self.tiles[indx] = tile;
        if !self.is_valid() {
            self.tiles[indx] = tmp;
        }
    }

    fn is_valid(&self) -> bool {
        for i in 0..9 {
            for j in 0..9 {
                let n = match self.tiles[i * 9 + j] {
                    Tile::Variable(n) | Tile::Const(n) => n,
                    Tile::Empty => continue,
                };
                for k in 0..9 {
                    match self.tiles[i * 9 + k] {
                        Tile::Variable(m) | Tile::Const(m) if n == m && j != k => return false,
                        _ => (),
                    }
                    match self.tiles[k * 9 + j] {
                        Tile::Variable(m) | Tile::Const(m) if n == m && i != k => return false,
                        _ => (),
                    }
                }
                let x0 = (j / 3) * 3;
                let y0 = (i / 3) * 3;
                for k in 0..3 {
                    for l in 0..3 {
                        match self.tiles[(y0 + k) * 9 + x0 + l] {
                            Tile::Variable(m) | Tile::Const(m)
                                if n == m && (y0 + k) * 9 + x0 + l != i * 9 + j =>
                            {
                                return false
                            }
                            _ => (),
                        }
                    }
                }
            }
        }
        true
    }

    pub fn step(&mut self) {
        let mut steps_per_frame = self.steps_per_frame;
        if self.steps_per_frame < 1.0 {
            if self.substeps < (1.0 / self.steps_per_frame) as u64 {
                self.substeps += 1;
                return;
            }
            self.substeps = 0;
            steps_per_frame = 1.0;
        }

        for i in 0.. {
            if i >= steps_per_frame as u64 && !self.solve_instantly {
                break;
            }
            if self.active_indx >= 81 {
                self.running = false;
                return;
            }
            self.step_count += 1;
            match self.tiles[self.active_indx] {
                Tile::Empty => {
                    self.tiles[self.active_indx] = Tile::Variable(1);
                    self.direction = Direction::Forward;
                }
                Tile::Const(_) => match self.direction {
                    Direction::Forward => self.active_indx += 1,
                    Direction::Backward => self.active_indx -= 1,
                },
                Tile::Variable(n) => {
                    if self.is_valid() && self.direction == Direction::Forward {
                        self.active_indx += 1;
                    } else {
                        if n == 9 {
                            self.tiles[self.active_indx] = Tile::Empty;
                            self.active_indx -= 1;
                            self.direction = Direction::Backward;
                        } else {
                            self.tiles[self.active_indx] = Tile::Variable(n + 1);
                            self.direction = Direction::Forward;
                        }
                    }
                }
            }
        }
    }
}