use macroquad::prelude::*;
use std::process;
use std::thread;
use std::time::Duration;

#[macroquad::main("Input")]
async fn main() {
    miniquad::prevent_double_click_bug_on_macos();

    let mut flip_flop = false;

    thread::sleep(Duration::new(2, 0));

    loop {
        if flip_flop {
            clear_background(LIGHTGRAY);
        } else {
            clear_background(DARKGRAY);
        }

        if is_mouse_button_pressed(MouseButton::Left) {
            flip_flop = !flip_flop;
        }

        if is_key_down(KeyCode::Escape) {
            process::exit(0);
        }

        next_frame().await;
    }
}
