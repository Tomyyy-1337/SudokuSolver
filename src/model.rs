use nannou::{color, prelude::*};
use crate::sudoku::{self, Tile};

pub struct Model {
    pub window_width: u32,
    pub window_height: u32,
    pub sudoku: sudoku::Sudoku,
    pub selected: Option<usize>,
    pub key_delay: f32,
}

impl Model {
    pub fn new(width: u32, height: u32) -> Self {
        Model {         
            window_width: width,
            window_height: height,
            selected: None,
            sudoku: sudoku::Sudoku::default(),
            key_delay: 0.0,
        }
    }

    pub fn draw_gui(&self, draw: &Draw) {
        let size = self.window_height.min(self.window_width) as f32 + 10.0;
        let ui_width = ((self.window_width as f32 - size) / 2.0 - 20.0).max(160.0).min(300.0);
        let title_size = (ui_width / 6.0) as u32;
        let sub_title_size = (ui_width / 8.0) as u32;
        let text_size = (ui_width / 12.0) as u32;

        let (x,mut y) = (
            size / 2.0 + ui_width as f32 / 1.9,
            size / 2.0,
        ); 
        add_label(draw, "Sudoku", x, &mut y, ui_width, title_size, color::WHITE);

        add_label(draw, "Solver:", x, &mut y, ui_width, sub_title_size, color::WHITE);
        add_label(draw, &format!("Running: {}", self.sudoku.running), x, &mut y, ui_width, text_size, color::WHITE);
        add_label(draw, &format!("Steps per frame: {:.2}", self.sudoku.steps_per_frame), x, &mut y, ui_width, text_size, color::WHITE);
        add_label(draw, &format!("Current Steps: {}", self.sudoku.step_count), x, &mut y, ui_width, text_size, color::WHITE);
        add_label(draw, &"[Space] Toggle solver", x, &mut y, ui_width, text_size, color::GREY);
        add_label(draw, &"[E] Clear Result", x, &mut y, ui_width, text_size, color::GREY);
        add_label(draw, &"[Up] Faster Simulation", x, &mut y, ui_width, text_size, color::GREY);
        add_label(draw, &"[Down] Slower Simulation", x, &mut y, ui_width, text_size, color::GREY);
        
        add_label(draw, "Difficulty:", x, &mut y, ui_width, sub_title_size, color::WHITE);
        add_label(draw, &format!("Selected: {}", self.sudoku.difficulty.to_string()), x, &mut y, ui_width, text_size, color::WHITE);
        add_label(draw, &"[Left] Easier Difficulty", x, &mut y, ui_width, text_size, color::GREY);
        add_label(draw, &"[Right] Harder Difficulty", x, &mut y, ui_width, text_size, color::GREY);
        add_label(draw, &"[R] Load new Sudoku", x, &mut y, ui_width, text_size, color::GREY);
        add_label(draw, &"[W] Clear Sudoku", x, &mut y, ui_width, text_size, color::GREY);
    }

    /// Tracks the elapsed time since the last frame key press.
    /// Returns true if the delay of 0.2 seconds has passed.
    pub fn key_delay_over(&mut self) -> bool {
        if self.key_delay < 0.2 {
            return false;
        }
        self.key_delay = 0.0;
        true
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

fn add_label(draw: &Draw, text: &str ,x: f32, y: &mut f32, ui_width: f32, font_size: u32, color: rgb::Rgb<color::encoding::Srgb, u8>) {
    *y -= font_size as f32 * 0.75;
    draw.text(text)
        .x_y(x, *y)
        .w(ui_width)
        .left_justify()
        .font_size(font_size)
        .color(color);
    *y -= font_size as f32 * 0.75;
}