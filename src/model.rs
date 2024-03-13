use std::collections::VecDeque;
use nannou::{color::{self, rgb::Rgba}, prelude::*};

use crate::sudoku::{self, Tile};
use crate::theme::{Theme, ThemeType};

#[derive(Default)]
pub struct Model {
    pub sudoku: sudoku::Sudoku,
    pub theme: Theme,
    pub window_width: u32,
    pub window_height: u32,
    pub offset: f32,
    pub selected: Option<usize>,
    size: f32,
    gui_width: f32,
    past_frametimes: VecDeque<f32>,
    past_frametimes_sum: f32,
    fps: f32,
    application_ticks: u64,
}

impl Model {
    pub fn new(width: u32, height: u32) -> Self {
        let mut model = Model::default();
        model.update_size(width, height);
        model.theme.update(ThemeType::Dark);
        model
    }

    pub fn draw(&self, draw: &Draw) {
        draw.background().color(self.theme.background_color);
        self.draw_grid(draw);
        self.draw_numbers(draw);
        self.draw_gui(draw);
    }

    pub fn update_size(&mut self, width: u32, height: u32) {
        self.application_ticks += 1;
        let size = height.min(width) as f32 - 10.0;
        let gui_width = ((width as f32 - self.size) - 10.0)
            .min(320.0)
            .min(self.size / 2.7)
            .max(120.0);
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

    pub fn update_past_frametimes(&mut self, time: f32) {
        self.past_frametimes.push_back(time);
        self.past_frametimes_sum += time;
        if self.past_frametimes.len() > 150 {
            self.past_frametimes_sum -= self.past_frametimes.pop_front().unwrap();
        }
        if self.application_ticks % 150 == 0 {
            self.fps = (1.0 / (self.past_frametimes_sum / self.past_frametimes.len() as f32)).round();
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
                        .color(self.theme.secondary_color);
                }
                Tile::Const(n) => {
                    draw.text(&n.to_string())
                        .x_y(x, y)
                        .font_size(self.size as u32 / 16)
                        .color(self.theme.primary_color);
                }
            }
        });
    }

    fn draw_grid(&self, draw: &Draw) {
        draw.rect()
            .x_y(0.0 - self.offset, 0.0)
            .z(0.0)
            .w_h(self.size, self.size)
            .color(self.theme.tile_color);
        for i in 0..=9 {
            let (color, z) = if i % 3 == 0 { 
                (self.theme.primary_color, 2.0) 
            } else { 
                (self.theme.secondary_color, 1.0) 
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
                .z(0.0)
                .color(color::rgba(255, 0, 0, self.theme.theme_alpha));
        } else if let Some(indx) = self.selected {
            let x = (indx % 9) as f32 * self.size / 9.0 - self.size / 2.0;
            let y = (indx / 9) as f32 * self.size / 9.0 - self.size / 2.0;
            let primary_color_with_alpha = Rgba {
                color: self.theme.primary_color,
                alpha: self.theme.theme_alpha,
            };
            draw.rect()
                .x_y(x + self.size / 18.0 - self.offset, y + self.size / 18.0)
                .w_h(self.size / 9.0, self.size / 9.0)
                .color(primary_color_with_alpha);
        }
    }

    fn draw_gui(&self, draw: &Draw) {
        let title_size = (self.gui_width / 6.0) as u32;
        let sub_title_size = (self.gui_width / 8.0) as u32;
        let text_size = (self.gui_width / 14.0) as u32;

        let mut y = self.size / 2.0;
        self.add_label(draw, "Sudoku", &mut y, title_size, self.theme.primary_color);

        self.add_label(draw, "Solver:", &mut y, sub_title_size, self.theme.primary_color);
        self.add_label(draw, &format!("Running: {}", self.sudoku.running), &mut y, text_size, self.theme.primary_color);
        self.add_label(draw, &format!("Steps per frame: {:.2}", self.sudoku.real_steps_per_frame), &mut y, text_size, self.theme.primary_color);
        self.add_label(draw, &format!("Steps per second: {:.0}", self.fps * self.sudoku.real_steps_per_frame as f32), &mut y, text_size, self.theme.primary_color);
        self.add_label(draw, &format!("Current Steps: {}", self.sudoku.step_count), &mut y, text_size, self.theme.primary_color);
        self.add_label(draw, &"[Space] Toggle solver", &mut y, text_size, self.theme.secondary_color);
        self.add_label(draw, &"[E] Clear Result", &mut y, text_size, self.theme.secondary_color);
        self.add_label(draw, &"[Up] Step faster", &mut y, text_size, self.theme.secondary_color);
        self.add_label(draw, &"[Down] Step slower", &mut y, text_size, self.theme.secondary_color);
        
        self.add_label(draw, "Difficulty:", &mut y, sub_title_size, self.theme.primary_color);
        self.add_label(draw, &format!("Selected: {}", self.sudoku.difficulty.to_string()), &mut y, text_size, self.theme.primary_color);
        self.add_label(draw, &"[Left] Easier Difficulty", &mut y, text_size, self.theme.secondary_color);
        self.add_label(draw, &"[Right] Harder Difficulty", &mut y, text_size, self.theme.secondary_color);
        self.add_label(draw, &"[R] Load new Sudoku", &mut y, text_size, self.theme.secondary_color);
        self.add_label(draw, &"[W] Clear Sudoku", &mut y, text_size, self.theme.secondary_color);

        self.add_label(draw, "Settings:", &mut y, sub_title_size, self.theme.primary_color);
        self.add_label(draw, &format!("Color Theme: {}", self.theme.theme_type.to_string()), &mut y, text_size, self.theme.primary_color);
        self.add_label(draw, &"[T] Change Color Theme", &mut y, text_size, self.theme.secondary_color);
        self.add_label(draw, &"[F11] Toggle Fullscreen", &mut y, text_size, self.theme.secondary_color);
        self.add_label(draw, &"[Escape] Close application", &mut y, text_size, self.theme.secondary_color);

    }

    fn add_label(
        &self,
        draw: &Draw,
        text: &str,
        y: &mut f32,
        font_size: u32,
        color: rgb::Rgb<color::encoding::Srgb, u8>,
    ) {
        let x = self.size / 2.0 - self.offset + self.gui_width / 2.0 + 15.0;
        *y -= font_size as f32 * 0.75;
        draw.text(text)
            .x_y(x, *y)
            .w(self.gui_width)
            .left_justify()
            .font_size(font_size)
            .color(color);
        *y -= font_size as f32 * 0.75;
    }
}
