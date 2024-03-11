use nannou::{color, prelude::*};
use nannou_egui::{self, egui, Egui};

use crate::sudoku::{self, Difficulty, Tile};

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
            ui.heading("Solver Settings:");
            if ui
                .checkbox(&mut self.sudoku.running, "Run solver")
                .clicked()
            {
                self.sudoku.reset_solver();
                self.sudoku.clear_variables();
            }
            ui.label("Steps per frame:");
            ui.add(
                egui::Slider::new(&mut self.sudoku.steps_per_frame, 0.01..=100000.0)
                .logarithmic(true),
            );
            let frame_rate: u32 = (1.0 / update.since_last.as_secs_f64()) as u32;
            ui.label(format!("Steps per second: {:.1}", self.sudoku.steps_per_frame * frame_rate as f64));
            ui.label(format!("Current Steps: {}", self.sudoku.step_count));
            ui.heading("Clear Options:");
            if ui.button("Clear All").clicked() {
                self.sudoku.tiles = vec![Tile::Empty; 81];
                self.sudoku.reset_solver()
            }
            if ui.button("Clear Result").clicked() {
                self.sudoku.clear_variables();
                self.sudoku.reset_solver()
            }

            if ui.button("Load random Sudoku").clicked() {
                self.sudoku.load_random();
            }
            ui.heading("Difficulty:");
            egui::ComboBox::new("Difficulty", "")
                .selected_text(self.sudoku.difficulty.to_string())
                .show_ui(ui, |ui| {
                    for kind in [
                        Difficulty::Easy,
                        Difficulty::Medium,
                        Difficulty::Hard,
                        Difficulty::VeryHard,
                    ] {
                        ui.selectable_value(&mut self.sudoku.difficulty, kind, kind.to_string());
                    }
                });
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

        if self.sudoku.running {
            let x = (self.sudoku.active_indx % 9) as f32 * size / 9.0 - size / 2.0;
            let y = (self.sudoku.active_indx / 9) as f32 * size / 9.0 - size / 2.0;
            draw.rect()
                .x_y(x + size / 18.0, y + size / 18.0)
                .w_h(size / 9.0, size / 9.0)
                .color(color::rgba(1.0, 0.0, 0.0, 0.1));
        } else if let Some(indx) = self.selected {
            let x = (indx % 9) as f32 * size / 9.0 - size / 2.0;
            let y = (indx / 9) as f32 * size / 9.0 - size / 2.0;
            draw.rect()
                .x_y(x + size / 18.0, y + size / 18.0)
                .w_h(size / 9.0, size / 9.0)
                .color(color::rgba(1.0, 1.0, 1.0, 0.05));
        }
    }
}
