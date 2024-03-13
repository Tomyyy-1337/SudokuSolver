use crate::model::Model;
use crate::sudoku::Tile;
use nannou::prelude::*;

pub fn mouse_moved(_app: &App, model: &mut Model, pos: Point2) {
    if model.sudoku.running {
        return;
    }
    let size = model.window_height.min(model.window_width) as f32 - 10.0;
    if (pos.x + model.offset).abs() < size / 2.0 && pos.y.abs() < size / 2.0 {
        let x = ((pos.x + model.offset + size / 2.0) / (size / 9.0)) as usize;
        let y = ((pos.y + size / 2.0) / (size / 9.0)) as usize;
        let selected_index = y * 9 + x;
        model.selected = Some(selected_index);
    } else {
        model.selected = None;
    }
}

pub fn handle_mouse_button_events(app: &App, window_height: u32, window_width: u32, model: &mut Model) {
    if model.sudoku.running {
        return;
    }
    let size = window_height.min(window_width) as f32;
    app.mouse.buttons.pressed().for_each(|button| match button {
        (MouseButton::Left | MouseButton::Right, v)
            if (v.x + model.offset).abs() < size / 2.0 && v.y.abs() < size / 2.0 =>
        {
            model.sudoku.clear_variables();
            model.sudoku.reset_solver();
            if let Some(selected) = model.selected {
                model.sudoku.try_insert(selected, Tile::Empty);
            }
        }
        _ => (),
    });
}

pub fn handle_key_pressed(app: &App, model: &mut Model, key: Key) {
    match key {
        Key::F11 => app
            .main_window()
            .set_fullscreen(!app.main_window().is_fullscreen()),
        Key::Return | Key::Space => {
            model.sudoku.reset_solver();
            model.sudoku.clear_variables();
            model.sudoku.running = !model.sudoku.running
        }
        Key::R => model.sudoku.load_random(),
        Key::E if !model.sudoku.running => {
            model.sudoku.clear_variables();
            model.sudoku.reset_solver();
        }
        Key::W if !model.sudoku.running => {
            model.sudoku.tiles = [Tile::Empty; 81];
            model.sudoku.reset_solver();
        }
        Key::Right => model.sudoku.difficulty = model.sudoku.difficulty.harder(),
        Key::Left => model.sudoku.difficulty = model.sudoku.difficulty.easier(),
        Key::T => model.theme.next(),
        _ => (),
    }
}

pub fn handle_continious_key_inputs(app: &App, model: &mut Model) {
    app.keys.down.iter().for_each(|key| match key {
        Key::Key1 | Key::Numpad1 => model.try_write_tile(Tile::Const(1)),
        Key::Key2 | Key::Numpad2 => model.try_write_tile(Tile::Const(2)),
        Key::Key3 | Key::Numpad3 => model.try_write_tile(Tile::Const(3)),
        Key::Key4 | Key::Numpad4 => model.try_write_tile(Tile::Const(4)),
        Key::Key5 | Key::Numpad5 => model.try_write_tile(Tile::Const(5)),
        Key::Key6 | Key::Numpad6 => model.try_write_tile(Tile::Const(6)),
        Key::Key7 | Key::Numpad7 => model.try_write_tile(Tile::Const(7)),
        Key::Key8 | Key::Numpad8 => model.try_write_tile(Tile::Const(8)),
        Key::Key9 | Key::Numpad9 => model.try_write_tile(Tile::Const(9)),
        Key::Key0 | Key::Numpad0 | Key::Back | Key::Delete => model.try_write_tile(Tile::Empty),
        Key::Up => model
            .sudoku
            .change_steps_per_frame(1.0 + 5.0 * app.duration.since_prev_update.as_secs_f32()),
        Key::Down => model.sudoku.change_steps_per_frame(
            1.0 / (1.0 + 5.0 * app.duration.since_prev_update.as_secs_f32()),
        ),
        _ => (),
    });
}

pub fn handle_mouse_wheel_events(
    _app: &App,
    model: &mut Model,
    dt: MouseScrollDelta,
    _phase: TouchPhase,
) {
    if let MouseScrollDelta::LineDelta(_x, y) = dt {
        model.sudoku.change_steps_per_frame(1.0 + 0.5 * y as f32);
    }
}

pub fn window_resized(_app: &App, model: &mut Model, dim: Vec2) {
    model.update_size(dim.x as u32, dim.y as u32);
}
