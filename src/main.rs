extern crate piston_window;
extern crate rand;

use piston_window::types::Color;
use piston_window::*;

mod draw;
mod game;
mod snake;
mod utils;

use draw::to_coord_u32;
use game::Game;

const BACK_COLOR: Color = [0.5, 0.5, 0.5, 1.0];

fn main() {
    let (width, height) = (20, 20);

    let mut window: PistonWindow =
        WindowSettings::new("Snake", [to_coord_u32(width), to_coord_u32(height)])
            .exit_on_esc(true)
            .build()
            .unwrap();

    let mut game = Game::new(width, height);

    while let Some(event) = window.next() {
        window.draw_2d(&event, |ctx, g, _| {
            // g -> 2d graphics
            clear(BACK_COLOR, g);
            game.draw(&ctx, g);
        });

        if let Some(Button::Keyboard(key)) = event.press_args() {
            game.key_pressed(key);
        }

        event.update(|arg| {
            game.update(arg.dt); // dt -> delta_time
        });
    }
}
