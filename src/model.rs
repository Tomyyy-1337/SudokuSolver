use std::usize;
use nannou::{color, prelude::*};
use nannou_egui::{self, egui, Egui};

pub struct Model {
    pub running: bool,
    pub egui: Egui,
    pub window_width: u32,
    pub window_height: u32,
    pub sudoku: Vec<Tile>,
    pub selected: Option<usize>,
    pub active_indx: usize,
    pub direction: Direction,
    pub step_count: u64,
    pub steps_per_frame: u64,
    pub solve_instantly: bool,
}

#[derive(PartialEq, Eq)]
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

impl Model {
    pub fn new(window: &std::cell::Ref<'_, Window>, width: u32, height: u32) -> Self {
        Model {
            running: false,
            egui: Egui::from_window(&window),
            window_width: width,
            window_height: height,
            selected: None,
            sudoku: vec![Tile::Empty; 81],
            active_indx: 0,
            direction: Direction::Forward,
            step_count: 0,
            steps_per_frame: 1,
            solve_instantly: false,
        }
    }

    pub fn try_insert(&mut self, indx: usize, tile: Tile) -> bool {
        let x = indx % 9;
        let y = indx / 9;
        let n = match tile {
            Tile::Variable(n) => n,
            Tile::Const(n) => n,
            _ => {
                self.sudoku[indx] = tile;
                return true
            },
        };

        for i in 0..9 {
            match self.sudoku[y * 9 + i] {
                Tile::Variable(m) if n == m && y * 9 + i != x + y * 9 => return false,
                Tile::Const(m) if n == m && y * 9 + i != x + y * 9=> return false,
                _ => (),
            }
            match self.sudoku[i * 9 + x] {
                Tile::Variable(m) if n == m && i * 9 + x != x + y * 9=> return false,
                Tile::Const(m) if n == m && i * 9 + x != x + y * 9=> return false,
                _ => (),
            }
        }
        let x0 = (x / 3) * 3;
        let y0 = (y / 3) * 3;
        for i in 0..3 {
            for j in 0..3 {
                match self.sudoku[(y0 + i) * 9 + x0 + j] {
                    Tile::Variable(m) if n == m && (y0 + i) * 9 + x0 + j != x + y * 9 => return false,
                    Tile::Const(m) if n == m && (y0 + i) * 9 + x0 + j != x + y * 9 => return false,
                    _ => (),
                }
            }
        }
        self.sudoku[indx] = tile;
        true
    }

    fn is_valid(&self) -> bool {
        for i in 0..9 {
            for j in 0..9 {
                let n = match self.sudoku[i * 9 + j] {
                    Tile::Variable(n) => n,
                    Tile::Const(n) => n,
                    Tile::Empty => continue,
                };
                for k in 0..9 {
                    if let Tile::Variable(m) = self.sudoku[i * 9 + k] {
                        if n == m && j != k {
                            return false
                        }
                    }
                    if let Tile::Variable(m) = self.sudoku[k * 9 + j] {
                        if n == m && i != k {
                            return false
                        }
                    }
                }
                let x0 = (j / 3) * 3;
                let y0 = (i / 3) * 3;
                for k in 0..3 {
                    for l in 0..3 {
                        if let Tile::Variable(m) = self.sudoku[(y0 + k) * 9 + x0 + l] {
                            if n == m && (y0 + k) * 9 + x0 + l != i * 9 + j {
                                return false
                            }
                        }
                    }
                }
            }
        }
        true
    }

    pub fn step(&mut self) {
        for i in 0.. {
            if i >= self.steps_per_frame && !self.solve_instantly {
                break
            }
            if self.active_indx >= 81 {
                self.running = false;
                return
            }
            self.step_count += 1;
            match self.sudoku[self.active_indx] {
                Tile::Empty => {
                    self.sudoku[self.active_indx] = Tile::Variable(1);
                    self.direction = Direction::Forward;
                },
                Tile::Const(_) => {
                    match self.direction {
                        Direction::Forward => self.active_indx += 1,
                        Direction::Backward => self.active_indx -= 1,
                    }
                },
                Tile::Variable(n) => {
                    if self.is_valid() && self.direction == Direction::Forward {
                        self.active_indx += 1;
                    } else {
                        if n == 9 {
                            self.sudoku[self.active_indx] = Tile::Empty;
                            self.active_indx -= 1;
                            self.direction = Direction::Backward;
                        } else {
                            self.sudoku[self.active_indx] = Tile::Variable(n + 1);
                            self.direction = Direction::Forward;
                        }
                    }
                }
            }
        }
    }
    
    pub fn update_gui(&mut self, update: Update) {
        self.egui.set_elapsed_time(update.since_start);
        let ctx = self.egui.begin_frame();
        egui::Window::new("Settings").show(&ctx, |ui| {
            ui.label("Toggle solver");
            if ui.checkbox(&mut self.running, "Running").clicked() {
                self.active_indx = 0;
                self.step_count = 0;
                self.direction = Direction::Forward;
                self.sudoku = self.sudoku.iter().map(|t| match t {
                    Tile::Variable(_) => Tile::Empty,
                    Tile::Const(n) => Tile::Const(*n),
                    Tile::Empty => Tile::Empty,
                }).collect();
            }
            ui.label(format!("Steps: {}", self.step_count));
            ui.checkbox(&mut self.solve_instantly, "Solve instantly");
            if !self.solve_instantly {
                ui.label("Steps per frame:");
                ui.add(egui::Slider::new(&mut self.steps_per_frame, 1..=100000).logarithmic(true));
            }
            ui.label("Clear");
            if ui.button("Clear All").clicked() {
                self.sudoku = vec![Tile::Empty; 81];
                self.active_indx = 0;
                self.step_count = 0;
                self.direction = Direction::Forward;
            }
            if ui.button("Clear Result").clicked() {
                self.sudoku = self.sudoku.iter().map(|t| match t {
                    Tile::Variable(_) => Tile::Empty,
                    Tile::Const(n) => Tile::Const(*n),
                    Tile::Empty => Tile::Empty,
                }).collect(); 
                self.active_indx = 0;
                self.step_count = 0;
                self.direction = Direction::Forward;
            }
            if ui.button("Clear selection").clicked() {
                self.selected = None;
            }
        });    
    }

    pub fn draw(&self, draw: &Draw) {
        draw.background().color(BLACK);
        let size = self.window_height.min(self.window_width) as f32;

        //draw Grid
        for i in 0..=9 {
            let color = if i % 3 == 0 {color::WHITE} else {color::GREY};
            let width = if i % 3 == 0 {2.0} else {1.0};
            draw.line()
                .start(pt2(-size / 2.0, size / 9.0 * i as f32 - size / 2.0))
                .end(pt2(size / 2.0, size / 9.0 * i as f32 - size / 2.0))
                .stroke_weight(width)
                .color(color);
            draw.line()
                .start(pt2(size / 9.0 * i as f32 - size / 2.0, -size / 2.0))
                .end(pt2(size / 9.0 * i as f32 - size / 2.0, size / 2.0))
                .stroke_weight(width)
                .color(color);
        }
        //draw selected
        if let Some(indx) = self.selected {
            let x = (indx % 9) as f32 * size / 9.0 - size / 2.0;
            let y = (indx / 9) as f32 * size / 9.0 - size / 2.0;
            draw.rect()
                .x_y(x + size / 18.0,y + size / 18.0)
                .w_h(size / 9.0, size / 9.0)
                .color(color::rgba(1.0, 1.0, 1.0, 0.05));
        }
        //draw numbers
        self.sudoku.iter()
            .enumerate()
            .for_each(|(i, t)| {
                let x = i % 9;
                let y = i / 9;
                match t {
                    Tile::Empty => (),
                    Tile::Variable(n) => {
                        draw.text(&n.to_string())
                            .x_y(size / 9.0 * (x as f32 + 0.5) - size / 2.0, size / 9.0 * (y as f32 + 0.5) - size / 2.04)
                            .font_size(size as u32 / 16)
                            .color(color::GREY);
                    },
                    Tile::Const(n) => {
                        draw.text(&n.to_string())
                            .x_y(size / 9.0 * (x as f32 + 0.5) - size / 2.0, size / 9.0 * (y as f32 + 0.5) - size / 2.04)
                            .font_size(size as u32 / 16)
                            .color(color::WHITE);
                    },
                }
            });
    }
}