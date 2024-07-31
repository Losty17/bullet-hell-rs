mod consts;
mod game_object;
mod player;

use crate::player::Player;
use raylib::{
    color::Color,
    drawing::{RaylibDraw, RaylibDrawHandle},
};

fn main() {
    let (mut rl, thread) = raylib::init().size(600, 800).title("bullet hell").build();
    rl.set_target_fps(60);

    let mut player = Player::new();

    while !rl.window_should_close() {
        player.movement(&rl);

        let mut d: RaylibDrawHandle = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        player.draw(&mut d);
    }
}
