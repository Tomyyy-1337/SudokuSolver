use nannou_egui::{self, egui, Egui};
use nannou::{color, prelude::*};
use crate::model::Model;
use crate::model::Tile;
use crate::model::Direction;

pub struct Gui {
    pub egui: Egui,
}

impl Gui {    
    pub fn update_gui(&mut self, update: Update, active_index: &mut usize) {
        self.egui.set_elapsed_time(update.since_start);
        let ctx = self.egui.begin_frame();
        egui::Window::new("Settings").show(&ctx, |ui| {
            ui.label("Toggle solver");
            if ui.checkbox(&mut model.running, "Running").clicked() {
                model.active_indx = 0;
                model.step_count = 0;
                model.direction = Direction::Forward;
                model.sudoku = model.sudoku.iter().map(|t| match t {
                    Tile::Variable(_) => Tile::Empty,
                    Tile::Const(n) => Tile::Const(*n),
                    Tile::Empty => Tile::Empty,
                }).collect();
            }
            ui.label(format!("Steps: {}", model.step_count));
            ui.checkbox(&mut model.solve_instantly, "Solve instantly");
            if !model.solve_instantly {
                ui.label("Steps per frame:");
                ui.add(egui::Slider::new(&mut model.steps_per_frame, 1..=100000).logarithmic(true));
            }
            ui.label("Clear");
            if ui.button("Clear All").clicked() {
                model.sudoku = vec![Tile::Empty; 81];
                model.active_indx = 0;
                model.step_count = 0;
                model.direction = Direction::Forward;
            }
            if ui.button("Clear Result").clicked() {
                model.sudoku = model.sudoku.iter().map(|t| match t {
                    Tile::Variable(_) => Tile::Empty,
                    Tile::Const(n) => Tile::Const(*n),
                    Tile::Empty => Tile::Empty,
                }).collect(); 
                model.active_indx = 0;
                model.step_count = 0;
                model.direction = Direction::Forward;
            }
            if ui.button("Clear selection").clicked() {
                model.selected = None;
            }
        });    
    }
}