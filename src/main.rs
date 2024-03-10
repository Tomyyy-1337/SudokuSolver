use nannou::prelude::*;

mod model;
use model::{Model, Tile};

fn main() {
    nannou::app(model)
        .update(update)
        .run();
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
    let window_width = app.window_rect().w() as u32;
    let window_height = app.window_rect().h() as u32;
    
    if window_width != model.window_width || window_height != model.window_height {
        model.window_width = window_width;
        model.window_height = window_height;
    }
    app.keys.down.iter().for_each(|key| {
        match key {
            Key::F11 => app.main_window().set_fullscreen(!app.main_window().is_fullscreen()),
            Key::Back => model.running = !model.running,
            Key::Key1 | Key::Key2 | Key::Key3 | Key::Key4 | Key::Key5 | Key::Key6 | Key::Key7 | Key::Key8 | Key::Key9 |
            Key::Numpad1 | Key::Numpad2 | Key::Numpad3 | Key::Numpad4 | Key::Numpad5 | Key::Numpad6 | Key::Numpad7 | Key::Numpad8 | Key::Numpad9 => {
                if let Some(selected) = model.selected {
                    let number = match key {
                        Key::Key1 | Key::Numpad1 => 1,
                        Key::Key2 | Key::Numpad2 => 2,
                        Key::Key3 | Key::Numpad3 => 3,
                        Key::Key4 | Key::Numpad4 => 4,
                        Key::Key5 | Key::Numpad5 => 5,
                        Key::Key6 | Key::Numpad6 => 6,
                        Key::Key7 | Key::Numpad7 => 7,
                        Key::Key8 | Key::Numpad8 => 8,
                        Key::Key9 | Key::Numpad9 => 9,
                        _ => panic!("This should never happen"),
                    };
                    if model.try_insert(selected, Tile::Const(number)) {
                        model.selected = None;
                    }
                }
            }
            _ => (),
        }
    });

    app.mouse.buttons.pressed().for_each(|button| {
        let size = window_height.min(window_width) as f32;
        match button {
            (MouseButton::Left, v) if v.x.abs() < size / 2.0 && v.y.abs() < size / 2.0 => {
                let x = ((v.x + size / 2.0) / (size / 9.0)) as usize;
                let y = ((v.y + size / 2.0) / (size / 9.0)) as usize;
                let selected_index = y * 9 + x;
                model.selected = Some(selected_index);
            }
            (MouseButton::Right, v) if v.x.abs() < size / 2.0 && v.y.abs() < size / 2.0 => {
                let x = ((v.x + size / 2.0) / (size / 9.0)) as usize;
                let y = ((v.y + size / 2.0) / (size / 9.0)) as usize;
                let selected_index = y * 9 + x;
                model.try_insert(selected_index, Tile::Empty);
            }
            _ => (),
            
        }
    });

    model.update_gui(update);

    if model.running {
        model.step();
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