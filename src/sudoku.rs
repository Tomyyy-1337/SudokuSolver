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

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Tile {
    Empty,
    SolverVariable(u8),
    PlayerVariable(u8),
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

#[derive(Clone, Copy, Debug)]
pub enum SolverState {
    Idle,
    Running,
    SolutionFound,
    NoSolution,
}

impl SolverState {
    pub fn to_string(&self) -> &str {
        match self {
            SolverState::Running => "Running",
            SolverState::Idle => "Idle",
            SolverState::SolutionFound => "Solution Found",
            SolverState::NoSolution => "No Solution",
        }
    }
}

#[derive(Clone)]
pub struct Sudoku {
    pub tiles: [Tile; 81],
    pub difficulty: Difficulty,
    pub active_indx: usize,
    pub state: SolverState,
    pub step_count: u64,
    pub real_steps_per_frame: f32,
    direction: Direction,
    steps_per_frame: f32,
    substeps: u8,
}


impl Default for Sudoku {
    fn default() -> Self {
        Sudoku {
            tiles: [Tile::Empty; 81],
            active_indx: 0,
            direction: Direction::Forward,
            state: SolverState::Idle,
            step_count: 0,
            steps_per_frame: 1.0,
            real_steps_per_frame: 1.0,
            substeps: 0,
            difficulty: Difficulty::Medium,
        }
    }
}

impl Sudoku {
    pub fn next_available_number(&self, indx: usize) -> Option<u8> {
        let available = self.avaliable_numbers(indx);
        let current = match self.tiles[indx] {
            Tile::SolverVariable(n) => n,
            _ => 0,
        };
        for i in current as u8 + 1..=9 {
            if available >> i & 1 == 0 {
                return Some(i);
            }
        }
        None
    }

    fn is_available(&self, indx: usize, n: u8) -> bool {
        let available = self.avaliable_numbers(indx);
        available >> n & 1 == 0
    }

    pub fn avaliable_numbers(&self, indx: usize) -> u16 {
        let mut seen: u16 = 0;
        let functions = [
            | indx: usize, i: usize | indx - indx % 9 + i,
            | indx: usize, i: usize | indx % 9 + i * 9,
            | indx: usize, i: usize | (indx % 9 ) / 3 * 3 + (indx / 9) / 3  * 27 + (i % 3) + (i / 3) * 9,
        ];
        for f in functions.iter() {
            for i in 0..9 {
                match self.tiles[f(indx, i)] {
                    Tile::SolverVariable(n) | Tile::Const(n) | Tile::PlayerVariable(n) => seen |= 1 << n,
                    _ => (),
                };
            }
        }
        seen
    }

    pub fn reset_solver(&mut self) {
        self.active_indx = 0;
        self.step_count = 0;
        self.direction = Direction::Forward;
        self.state = SolverState::Idle;
    }

    pub fn clear_variables(&mut self) {
        for tile in self.tiles.iter_mut() {
            if let Tile::SolverVariable(_) = tile {
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
    pub fn change_steps_per_frame(&mut self, mult: f32) {
        self.steps_per_frame = (self.steps_per_frame * mult).max(0.005).min(100000.0);
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
                .collect::<Vec<Tile>>()
                .try_into()
                .unwrap(),
            active_indx: 0,
            direction: Direction::Forward,
            state: SolverState::Idle,
            step_count: 0,
            substeps: 0,
            ..*self
        }
    }

    pub fn is_running(&self) -> bool {
        matches!(self.state, SolverState::Running)
    }

    pub fn try_insert(&mut self, indx: usize, tile: Tile) {
        if let Tile::Const(_) = self.tiles[indx] {
            return;
        }
        match tile {
            Tile::SolverVariable(n) | Tile::Const(n) | Tile::PlayerVariable(n) if self.is_available(indx, n) => self.tiles[indx] = tile,
            Tile::Empty => self.tiles[indx] = tile,
            _ => (),
        }
    }

    fn get_steps(&mut self) -> u32 {
        if self.steps_per_frame < 1.0 {
            if self.substeps < (1.0 / self.steps_per_frame) as u8 {
                self.substeps += 1;
                0
            } else {
                self.substeps = 0;
                1
            }
        } else {
            self.real_steps_per_frame as u32
        }
    }
    fn solution_possible(&self) -> bool {
        let functions = [
            | indx: usize, i: usize | indx - indx % 9 + i,
            | indx: usize, i: usize | indx % 9 + i * 9,
            | indx: usize, i: usize | (indx % 9 ) / 3 * 3 + (indx / 9) / 3  * 27 + (i % 3) + (i / 3) * 9,
        ];
        functions.iter().all(|f| 
            (0..9)
                .map(|i| f(self.active_indx, i))
                .all(|tile| match self.tiles[tile] {
                    Tile::Empty => self.avaliable_numbers(tile) != 0b1111111110,
                    _ => true,
                })
        )
    }

    pub fn step(&mut self) {
        for _ in 0..self.get_steps() {
            if self.active_indx >= 81 {
                self.state = if !self.tiles.contains(&Tile::Empty) {
                    SolverState::SolutionFound
                } else {
                    SolverState::NoSolution
                };
                return;
            }
            self.step_count += 1;
            let next_number = self.next_available_number(self.active_indx);
            match self.tiles[self.active_indx] {
                Tile::Const(_) | Tile::PlayerVariable(_) => match self.direction {
                    Direction::Forward => self.active_indx += 1,
                    Direction::Backward => self.active_indx -= 1,
                }
                Tile::SolverVariable(_) if self.direction == Direction::Forward => {
                    self.active_indx += 1
                }
                Tile::SolverVariable(_) | Tile::Empty if next_number.is_some() => {
                    self.tiles[self.active_indx] = Tile::SolverVariable(next_number.unwrap());
                    match self.solution_possible() {
                        true => self.direction = Direction::Forward,
                        false => self.direction = Direction::Backward,
                    }
                }
                _ => {
                    self.tiles[self.active_indx] = Tile::Empty;
                    self.active_indx -= 1;
                    self.direction = Direction::Backward;
                },
            }
        }
    }
}
