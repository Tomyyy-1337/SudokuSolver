use nannou::{color, prelude::*};
use nannou_egui::{self, egui, Egui};

use crate::sudoku::{self, Direction, Tile};

pub struct Model {
    pub egui: Egui,
    pub window_width: u32,
    pub window_height: u32,
    pub sudoku: sudoku::Sudoku,
    pub selected: Option<usize>,
}

impl Model {
    pub fn new(window: &std::cell::Ref<'_, Window>, width: u32, height: u32) -> Self {
        Model {
            egui: Egui::from_window(&window),
            window_width: width,
            window_height: height,
            selected: None,
            sudoku: sudoku::Sudoku::default(),
        }
    }

    pub fn update_gui(&mut self, update: Update) {
        self.egui.set_elapsed_time(update.since_start);
        let ctx = self.egui.begin_frame();
        egui::Window::new("Settings").show(&ctx, |ui| {
            ui.label("Toggle solver");
            if ui.checkbox(&mut self.sudoku.running, "Running").clicked() {
                self.sudoku.active_indx = 0;
                self.sudoku.step_count = 0;
                self.sudoku.direction = Direction::Forward;
                self.sudoku.tiles = self
                    .sudoku
                    .tiles
                    .iter()
                    .map(|t| match t {
                        Tile::Variable(_) => Tile::Empty,
                        Tile::Const(n) => Tile::Const(*n),
                        Tile::Empty => Tile::Empty,
                    })
                    .collect();
            }
            ui.label(format!("Steps: {}", self.sudoku.step_count));
            ui.checkbox(&mut self.sudoku.solve_instantly, "Solve instantly");
            if !self.sudoku.solve_instantly {
                ui.label("Steps per frame:");
                ui.add(
                    egui::Slider::new(&mut self.sudoku.steps_per_frame, 1..=100000)
                        .logarithmic(true),
                );
            }
            ui.label("Clear");
            if ui.button("Clear All").clicked() {
                self.sudoku.tiles = vec![Tile::Empty; 81];
                self.sudoku.active_indx = 0;
                self.sudoku.step_count = 0;
                self.sudoku.direction = Direction::Forward;
            }
            if ui.button("Clear Result").clicked() {
                self.sudoku.tiles = self
                    .sudoku
                    .tiles
                    .iter()
                    .map(|t| match t {
                        Tile::Variable(_) => Tile::Empty,
                        Tile::Const(n) => Tile::Const(*n),
                        Tile::Empty => Tile::Empty,
                    })
                    .collect();
                self.sudoku.active_indx = 0;
                self.sudoku.step_count = 0;
                self.sudoku.direction = Direction::Forward;
            }
        });
    }

    pub fn draw(&self, draw: &Draw) {
        let size = self.window_height.min(self.window_width) as f32 - 10.0;

        draw.background().color(BLACK);
        self.draw_grid(size, draw);
        self.draw_numbers(draw, size);
    }

    fn draw_numbers(&self, draw: &Draw, size: f32) {
        self.sudoku.tiles.iter().enumerate().for_each(|(i, t)| {
            let x = i % 9;
            let y = i / 9;
            match t {
                Tile::Empty => (),
                Tile::Variable(n) => {
                    draw.text(&n.to_string())
                        .x_y(
                            size / 9.0 * (x as f32 + 0.5) - size / 2.0,
                            size / 9.0 * (y as f32 + 0.5) - size / 2.04,
                        )
                        .font_size(size as u32 / 16)
                        .color(color::GREY);
                }
                Tile::Const(n) => {
                    draw.text(&n.to_string())
                        .x_y(
                            size / 9.0 * (x as f32 + 0.5) - size / 2.0,
                            size / 9.0 * (y as f32 + 0.5) - size / 2.04,
                        )
                        .font_size(size as u32 / 16)
                        .color(color::WHITE);
                }
            }
        });
    }

    fn draw_grid(&self, size: f32, draw: &Draw) {
        for i in 0..=9 {
            let color = if i % 3 == 0 {
                color::WHITE
            } else {
                color::GREY
            };
            let width = if i % 3 == 0 { 2.0 } else { 1.0 };
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
        if let Some(indx) = self.selected {
            let x = (indx % 9) as f32 * size / 9.0 - size / 2.0;
            let y = (indx / 9) as f32 * size / 9.0 - size / 2.0;
            draw.rect()
                .x_y(x + size / 18.0, y + size / 18.0)
                .w_h(size / 9.0, size / 9.0)
                .color(color::rgba(1.0, 1.0, 1.0, 0.05));
        }
    }
}
