mod consts;
mod objects;

use std::thread;

use consts::Consts;
use objects::{Player, TestEnemy};
use raylib::{
    color::Color,
    drawing::{RaylibDraw, RaylibDrawHandle},
};

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(Consts::SCREEN_WIDTH, Consts::SCREEN_HEIGHT)
        .title("bullet hell")
        .build();

    rl.set_target_fps(60);

    let mut player = Player::new();
    let mut test_enemy = TestEnemy::new(
        Consts::SCREEN_WIDTH / 2,
        Consts::SCREEN_HEIGHT / 2 - 100,
        20,
        20,
        0.0,
    );

    let mut projectiles = test_enemy.update(&rl, Some(&player.game_object));
    let mut counter = 0;
    while !rl.window_should_close() {
        if counter % 60 == 0 {
            // update once every second
            projectiles.append(&mut test_enemy.update(&rl, Some(&player.game_object)));
            projectiles.retain(|p| {
                p.game_object.x > 0
                    && p.game_object.x < Consts::SCREEN_WIDTH
                    && p.game_object.y > 0
                    && p.game_object.y < Consts::SCREEN_HEIGHT
            });
        }

        player.movement(&rl);

        projectiles.iter_mut().for_each(|p| p.update());

        let mut d: RaylibDrawHandle = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        player.draw(&mut d);
        test_enemy.draw(&mut d);
        projectiles.iter().for_each(|p| p.draw(&mut d));
        counter += 1;

        d.draw_fps(0, 0);
    }
}
