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
    let width = 1000;
    let height = 800;
    app.new_window()
        .size(width, height)
        .mouse_wheel(events::handle_mouse_wheel_events)
        .view(view)
        .build()
        .unwrap();

    Model::new(width, height)
}

fn update(app: &App, model: &mut Model, _update: Update) {
    model.update_size(app.window_rect().w() as u32, app.window_rect().h() as u32);
    events::handle_keyboard_events(app, model);
    events::handle_mouse_events(app, model.window_height, model.window_width, model);

    model.update_past_frametimes(app.duration.since_prev_update.as_secs_f32());
    if model.sudoku.running {
        model.sudoku.step();
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    model.draw(&draw);
    draw.to_frame(app, &frame).unwrap();
}