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

    pub fn harder(&self) -> Self {
        match self {
            Difficulty::Easy => Difficulty::Medium,
            Difficulty::Medium => Difficulty::Hard,
            Difficulty::Hard => Difficulty::VeryHard,
            Difficulty::VeryHard => Difficulty::VeryHard,
        }
    }

    pub fn easier(&self) -> Self {
        match self {
            Difficulty::Easy => Difficulty::Easy,
            Difficulty::Medium => Difficulty::Easy,
            Difficulty::Hard => Difficulty::Medium,
            Difficulty::VeryHard => Difficulty::Hard,
        }
    }
}

#[derive(Clone)]
pub struct Sudoku {
    pub tiles: Vec<Tile>,
    pub active_indx: usize,
    pub running: bool,
    pub step_count: u64,
    pub steps_per_frame: f64,
    pub real_steps_per_frame: f64,
    pub difficulty: Difficulty,
    direction: Direction,
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
            real_steps_per_frame: 1.0,
            substeps: 0,
            difficulty: Difficulty::Medium,
        }
    }
}

impl Sudoku {
    /// Checks if the current state of the sudoku is valid.
    /// A State is valid if no number is repeated in any row, column or 3x3 square.
    pub fn is_valid(&self) -> bool {
        !self.check_seen(|i, j| i + j * 9)
            && !self.check_seen(|i, j| i * 9 + j)
            && !self.check_seen(|i, j| (i % 3) * 3 + (i / 3) * 27 + (j % 3) + (j / 3) * 9)
    }

    /// Checks if the current row, column or 3x3 square has any repeated numbers.
    /// Returns true if there are any repeated numbers, false otherwise.
    /// The function takes a closure that returns the index of the tile to check.
    fn check_seen(&self, function: fn(usize, usize) -> usize) -> bool {
        let mut seen: u16;
        for i in 0..9 {
            seen = 0;
            for indx in (0..9).map(|j| function(i, j)) {
                match self.tiles[indx] {
                    Tile::Variable(n) | Tile::Const(n) if seen >> n & 1 == 1 => return true,
                    Tile::Variable(n) | Tile::Const(n) => seen |= 1 << n,
                    _ => (),
                }
            }
        }
        false
    }

    pub fn reset_solver(&mut self) {
        self.active_indx = 0;
        self.step_count = 0;
        self.direction = Direction::Forward;
    }

    pub fn clear_variables(&mut self) {
        for tile in self.tiles.iter_mut() {
            if let Tile::Variable(_) = tile {
                *tile = Tile::Empty;
            }
        }
    }

    /// Loads a random Sudoku from the included list of sudokus
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

    /// Changes the number of steps per frame by a multiplier.
    /// Steps per frame is clamped between 0.01 and 100000.
    pub fn change_steps_per_frame(&mut self, mult: f64) {
        self.steps_per_frame = (self.steps_per_frame * mult).max(0.01).min(100000.0);
        if self.steps_per_frame < 1.0 {
            self.real_steps_per_frame = 1.0 / ((1.0 / self.steps_per_frame).floor() + 1.0);
        } else {
            self.real_steps_per_frame = self.steps_per_frame.floor();
        }
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
            substeps: 0,
            ..*self
        }
    }

    /// Tries to insert a tile at the given index.
    /// If the insertion results in an invalid state, the tile is not inserted.
    pub fn try_insert(&mut self, indx: usize, tile: Tile) {
        let tmp = self.tiles[indx];
        self.tiles[indx] = tile;
        if !self.is_valid() {
            self.tiles[indx] = tmp;
        }
    }

    /// Backtracking Sudoku solver.
    /// Itteraively solves the sudoku by trying to insert a number at each empty tile.
    /// If the insertion results in an invalid state, the tile is reset and the next number is tried.
    /// If no number can be inserted, the function backtracks to the previous tile and tries the next number.
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

        for _ in 0..steps_per_frame as u32 {
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
                Tile::Variable(_) if self.is_valid() && self.direction == Direction::Forward => {
                    self.active_indx += 1
                }
                Tile::Variable(n) if n == 9 => {
                    self.tiles[self.active_indx] = Tile::Empty;
                    self.active_indx -= 1;
                    self.direction = Direction::Backward;
                }
                Tile::Variable(n) => {
                    self.tiles[self.active_indx] = Tile::Variable(n + 1);
                    self.direction = Direction::Forward;
                }
            }
        }
    }
}