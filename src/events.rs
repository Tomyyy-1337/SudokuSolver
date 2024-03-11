use nannou::prelude::*;
use crate::model::Model;
use crate::sudoku::Tile;

pub fn handle_mouse_events(app: &App, window_height: u32, window_width: u32, model: &mut Model) {
    if model.sudoku.running {
        return;
    }
    let mouse_pos = app.mouse.position();
    let size = window_height.min(window_width) as f32 - 10.0;
    if mouse_pos.x.abs() < size / 2.0 && mouse_pos.y.abs() < size / 2.0 {
        let x = ((mouse_pos.x + size / 2.0) / (size / 9.0)) as usize;
        let y = ((mouse_pos.y + size / 2.0) / (size / 9.0)) as usize;
        let selected_index = y * 9 + x;
        model.selected = Some(selected_index);
    } else {
        model.selected = None;
    }

    let size = window_height.min(window_width) as f32;
    app.mouse.buttons.pressed().for_each(|button| match button {
        (MouseButton::Left | MouseButton::Right, v) if v.x.abs() < size / 2.0 && v.y.abs() < size / 2.0 => {
            model.sudoku.clear_variables();
            model.sudoku.reset_solver();
            if let Some(selected) = model.selected {
                model.sudoku.try_insert(selected, Tile::Empty);
            }
        }
        _ => (),
    });
}

pub fn handle_keyboard_events(app: &App, model: &mut Model) {
    model.key_delay += app.duration.since_prev_update.as_secs_f32();
    app.keys.down.iter().for_each(|key| match key {
        Key::F11 if model.key_delay_over() => 
            app.main_window().set_fullscreen(!app.main_window().is_fullscreen()),
        Key::Return | Key::Space if model.key_delay_over() => {
            model.sudoku.reset_solver();
            model.sudoku.clear_variables();
            model.sudoku.running = !model.sudoku.running
        },
        Key::Back | Key::Delete => if let Some(selected) = model.selected {
            model.sudoku.try_insert(selected, Tile::Empty);
        },
        Key::Key1 | Key::Numpad1 => write_number(model, 1),
        Key::Key2 | Key::Numpad2 => write_number(model, 2),
        Key::Key3 | Key::Numpad3 => write_number(model, 3),
        Key::Key4 | Key::Numpad4 => write_number(model, 4),
        Key::Key5 | Key::Numpad5 => write_number(model, 5),
        Key::Key6 | Key::Numpad6 => write_number(model, 6),
        Key::Key7 | Key::Numpad7 => write_number(model, 7),
        Key::Key8 | Key::Numpad8 => write_number(model, 8),
        Key::Key9 | Key::Numpad9 => write_number(model, 9),
        Key::R if model.key_delay_over() => model.sudoku.load_random(),
        Key::C if model.key_delay_over() => model.sudoku.clear_variables(),
        Key::Up => model.sudoku.change_steps_per_frame(1.0 + 5.0 * app.duration.since_prev_update.as_secs_f64()),
        Key::Down => model.sudoku.change_steps_per_frame(1.0 / (1.0 + 5.0 * app.duration.since_prev_update.as_secs_f64())),
        Key::Right if model.key_delay_over() => model.sudoku.difficulty = model.sudoku.difficulty.harder(),
        Key::Left if model.key_delay_over() => model.sudoku.difficulty = model.sudoku.difficulty.easier(),
        _ => (),
    });
}

/// Writes a number to the sudoku if a tile is selected
fn write_number(model: &mut Model, number: u8) {
    if model.sudoku.running {
        return;
    }
    model.sudoku.clear_variables();
    model.sudoku.reset_solver();
    if let Some(selected) = model.selected {
        model
            .sudoku
            .try_insert(selected, Tile::Const(number));
    }
}

pub fn update_size(app: &App, model: &mut Model) {
    let window_width = app.window_rect().w() as u32;
    let window_height = app.window_rect().h() as u32;
    if window_width != model.window_width || window_height != model.window_height {
        model.window_width = window_width;
        model.window_height = window_height;
    }
}