use std::collections::VecDeque;

use crate::sudoku::{self, Tile};
use nannou::{color, prelude::*};

#[derive(Default)]
pub struct Model {
    pub window_width: u32,
    pub window_height: u32,
    pub size: f32,
    pub gui_width: f32,
    pub sudoku: sudoku::Sudoku,
    pub selected: Option<usize>,
    pub key_delay: f32,
    pub offset: f32,
    pub past_frametimes: VecDeque<f32>,
}

impl Model {
    pub fn new(width: u32, height: u32) -> Self {
        let mut model = Model::default();
        model.update_size(width, height);
        model
    }

    pub fn draw(&self, draw: &Draw) {
        draw.background().color(BLACK);
        self.draw_grid(draw);
        self.draw_numbers(draw);
        self.draw_gui(draw);
    }

    pub fn update_size(&mut self, width: u32, height: u32) {
        let size = height.min(width) as f32 - 10.0;
        let gui_width = ((width as f32 - self.size) - 10.0)
            .max(140.0)
            .min(320.0)
            .min(self.size / 2.2);
        let offset = if size + gui_width > width as f32 - 20.0 {
            gui_width / 2.0 - (size + gui_width - width as f32) / 2.0 - 5.0
        } else {
            gui_width / 2.0
        };
        self.size = size;
        self.gui_width = gui_width;
        self.window_width = width;  
        self.window_height = height;
        self.offset = offset;
    }

    pub fn try_write_tile(&mut self, tile: Tile) {
        if self.sudoku.running {
            return;
        }
        self.sudoku.clear_variables();
        self.sudoku.reset_solver();
        if let Some(selected) = self.selected {
            self.sudoku.try_insert(selected, tile);
        }
    }

    pub fn key_delay_over(&mut self) -> bool {
        if self.key_delay < 0.2 {
            return false;
        }
        self.key_delay = 0.0;
        true
    }

    pub fn update_past_frametimes(&mut self, time: f32) {
        self.past_frametimes.push_back(time);
        if self.past_frametimes.len() > 100 {
            self.past_frametimes.pop_front();
        }
    }

    fn draw_numbers(&self, draw: &Draw) {
        self.sudoku.tiles.iter().enumerate().for_each(|(i, t)| {
            let x = self.size / 9.0 * ((i % 9) as f32 + 0.5) - self.size / 2.0 - self.offset;
            let y = self.size / 9.0 * ((i / 9) as f32 + 0.5) - self.size / 2.04;
            match t {
                Tile::Empty => (),
                Tile::Variable(n) => {
                    draw.text(&n.to_string())
                        .x_y(x, y)
                        .font_size(self.size as u32 / 16)
                        .color(color::GREY);
                }
                Tile::Const(n) => {
                    draw.text(&n.to_string())
                        .x_y(x, y)
                        .font_size(self.size as u32 / 16)
                        .color(color::WHITE);
                }
            }
        });
    }

    fn draw_grid(&self, draw: &Draw) {
        for i in 0..=9 {
            let (color, z) = if i % 3 == 0 { 
                (color::WHITE, 2.0) 
            } else { 
                (color::GREY, 1.0) 
            };
            let width = if i % 3 == 0 { 2.0 } else { 1.0 };
            draw.line()
                .start(pt2(
                    -self.size / 2.0 - self.offset,
                    self.size / 9.0 * i as f32 - self.size / 2.0,
                ))
                .end(pt2(
                    self.size / 2.0 - self.offset,
                    self.size / 9.0 * i as f32 - self.size / 2.0,
                ))
                .z(z)
                .stroke_weight(width)
                .color(color);
            draw.line()
                .start(pt2(
                    self.size / 9.0 * i as f32 - self.size / 2.0 - self.offset,
                    -self.size / 2.0,
                ))
                .end(pt2(
                    self.size / 9.0 * i as f32 - self.size / 2.0 - self.offset,
                    self.size / 2.0,
                ))
                .z(z)
                .stroke_weight(width)
                .color(color);
        }

        if self.sudoku.running {
            let x = (self.sudoku.active_indx % 9) as f32 * self.size / 9.0 - self.size / 2.0;
            let y = (self.sudoku.active_indx / 9) as f32 * self.size / 9.0 - self.size / 2.0;
            draw.rect()
                .x_y(x + self.size / 18.0 - self.offset, y + self.size / 18.0)
                .w_h(self.size / 9.0, self.size / 9.0)
                .color(color::rgba(1.0, 0.0, 0.0, 0.1));
        } else if let Some(indx) = self.selected {
            let x = (indx % 9) as f32 * self.size / 9.0 - self.size / 2.0;
            let y = (indx / 9) as f32 * self.size / 9.0 - self.size / 2.0;
            draw.rect()
                .x_y(x + self.size / 18.0 - self.offset, y + self.size / 18.0)
                .w_h(self.size / 9.0, self.size / 9.0)
                .color(color::rgba(1.0, 1.0, 1.0, 0.05));
        }
    }

    pub fn draw_gui(&self, draw: &Draw) {
        let title_size = (self.gui_width / 6.0) as u32;
        let sub_title_size = (self.gui_width / 8.0) as u32;
        let text_size = (self.gui_width / 14.0) as u32;

        let (x, mut y) = (
            self.size / 2.0 - self.offset + self.gui_width / 2.0 + 15.0,
            self.size / 2.0,
        );
        let framerate = 100.0 / self.past_frametimes.iter().sum::<f32>();
        Model::add_label(draw, "Sudoku", x, &mut y, self.gui_width, title_size, color::WHITE);
        Model::add_label(draw, "Solver:", x, &mut y, self.gui_width, sub_title_size, color::WHITE);
        Model::add_label(draw, &format!("Running: {}", self.sudoku.running), x, &mut y, self.gui_width, text_size, color::WHITE);
        Model::add_label(draw, &format!("Steps per frame: {:.2}", self.sudoku.real_steps_per_frame), x, &mut y, self.gui_width, text_size, color::WHITE);
        Model::add_label(draw, &format!("Steps per second: {:.0}", framerate.round() * self.sudoku.real_steps_per_frame as f32), x, &mut y, self.gui_width, text_size, color::WHITE);
        Model::add_label(draw, &format!("Current Steps: {}", self.sudoku.step_count), x, &mut y, self.gui_width, text_size, color::WHITE);
        Model::add_label(draw, &"[Space] Toggle solver", x, &mut y, self.gui_width, text_size, color::GREY);
        Model::add_label(draw, &"[E] Clear Result", x, &mut y, self.gui_width, text_size, color::GREY);
        Model::add_label(draw, &"[Up] Step faster", x, &mut y, self.gui_width, text_size, color::GREY);
        Model::add_label(draw, &"[Down] Step slower", x, &mut y, self.gui_width, text_size, color::GREY);
        Model::add_label(draw, "Difficulty:", x, &mut y, self.gui_width, sub_title_size, color::WHITE);
        Model::add_label(draw, &format!("Selected: {}", self.sudoku.difficulty.to_string()), x, &mut y, self.gui_width, text_size, color::WHITE);
        Model::add_label(draw, &"[Left] Easier Difficulty", x, &mut y, self.gui_width, text_size, color::GREY);
        Model::add_label(draw, &"[Right] Harder Difficulty", x, &mut y, self.gui_width, text_size, color::GREY);
        Model::add_label(draw, &"[R] Load new Sudoku", x, &mut y, self.gui_width, text_size, color::GREY);
        Model::add_label(draw, &"[W] Clear Sudoku", x, &mut y, self.gui_width, text_size, color::GREY);
    }

    fn add_label(
        draw: &Draw,
        text: &str,
        x: f32,
        y: &mut f32,
        ui_width: f32,
        font_size: u32,
        color: rgb::Rgb<color::encoding::Srgb, u8>,
    ) {
        *y -= font_size as f32 * 0.75;
        draw.text(text)
            .x_y(x, *y)
            .w(ui_width)
            .left_justify()
            .font_size(font_size)
            .color(color);
        *y -= font_size as f32 * 0.75;
    }
}
