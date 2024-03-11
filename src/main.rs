// #![windows_subsystem = "windows"]
use nannou::prelude::*;

mod events;
mod model;
mod sudoku;
use model::Model;

fn main() {
    nannou::app(model).update(update).run();
}

fn model(app: &App) -> Model {
    let width = 1200;
    let height = 800;
    let window_id = app
        .new_window()
        .size(width, height)
        .view(view)
        .raw_event(raw_window_event)
        .build()
        .unwrap();
    let window = app.window(window_id).unwrap();

    Model::new(&window, width, height)
}

fn update(app: &App, model: &mut Model, update: Update) {
    events::update_size(app, model);
    events::handle_keyboard_events(app, model);
    events::handle_mouse_events(app, model.window_height, model.window_width, model);

    model.update_gui(update);

    if model.sudoku.running {
        model.sudoku.step();
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    model.draw(&draw);
    draw.to_frame(app, &frame).unwrap();
    model.egui.draw_to_frame(&frame).unwrap();
}

fn raw_window_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    model.egui.handle_raw_event(event);
}