use rand::seq::IteratorRandom;

const EASY: &str = include_str!("../input/Sudoku_easy.sdm");
const MEDIUM: &str = include_str!("../input/Sudoku_medium.sdm");
const HARD: &str = include_str!("../input/Sudoku_hard.sdm");
const VERY_HARD: &str = include_str!("../input/Top_50K_Toughest.sdm");

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

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
    VeryHard,
}

impl Difficulty {
    pub fn to_string(&self) -> &str {
        match self {
            Difficulty::Easy => "Easy",
            Difficulty::Medium => "Medium",
            Difficulty::Hard => "Hard",
            Difficulty::VeryHard => "Very Hard",
        }
    }
}

#[derive(Clone)]
pub struct Sudoku {
    pub tiles: Vec<Tile>,
    pub active_indx: usize,
    pub direction: Direction,
    pub running: bool,
    pub step_count: u64,
    pub steps_per_frame: f64,
    substeps: u64,
    pub difficulty: Difficulty,
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
            substeps: 0,
            difficulty: Difficulty::Medium,
        }
    }
}

impl Sudoku {
    pub fn reset_solver(&mut self) {
        self.active_indx = 0;
        self.step_count = 0;
        self.direction = Direction::Forward;
    }

    pub fn load_random(&mut self) {
        let list = match self.difficulty {
            Difficulty::Easy => EASY,
            Difficulty::Medium => MEDIUM,
            Difficulty::Hard => HARD,
            Difficulty::VeryHard => VERY_HARD,
        };
        let random_line = list.lines().choose(&mut rand::thread_rng()).unwrap();
        *self = self.from_line(random_line);
    }

    fn from_line(&self, line: &str) -> Self {
        Sudoku {
            tiles: line
                .chars()
                .map(|c| match c {
                    '1'..='9' => Tile::Const(c.to_digit(10).unwrap() as u8),
                    '0' => Tile::Empty,
                    _ => panic!("Invalid character in input"),
                })
                .collect(),
            active_indx: 0,
            direction: Direction::Forward,
            running: false,
            step_count: 0,
            steps_per_frame: self.steps_per_frame,
            substeps: 0,
            difficulty: self.difficulty,
        }
    }

    pub fn clear_variables(&mut self) {
        for tile in self.tiles.iter_mut() {
            if let Tile::Variable(_) = tile {
                *tile = Tile::Empty;
            }
        }
    }

    pub fn try_insert(&mut self, indx: usize, tile: Tile) {
        let tmp = self.tiles[indx];
        self.tiles[indx] = tile;
        if !self.is_valid() {
            self.tiles[indx] = tmp;
        }
    }

    fn is_valid(&self) -> bool {
        if self.check_seen(|i, j| i + j * 9)
            || self.check_seen(|i, j| i * 9 + j)
            || self.check_seen(|i, j| (i % 3) * 3 + (i / 3) * 27 + (j % 3) + (j / 3) * 9)
        {
            return false;
        }
        true
    }

    fn check_seen(&self, function: fn(usize, usize) -> usize) -> bool {
        let mut seen: u16;
        for i in 0..9 {
            seen = 0;
            for j in 0..9 {
                let indx = function(i, j);
                match self.tiles[indx] {
                    Tile::Variable(n) | Tile::Const(n) if seen >> n & 1 == 1 => return true,
                    Tile::Variable(n) | Tile::Const(n) => seen |= 1 << n,
                    _ => (),
                }
            }
        }
        false
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
            if i >= steps_per_frame as u64 {
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