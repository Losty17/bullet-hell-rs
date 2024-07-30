use raylib::{
    color::Color,
    consts::KeyboardKey,
    drawing::{RaylibDraw, RaylibDrawHandle},
    RaylibHandle,
};

const SCREEN_WIDTH: u32 = 600;
const SCREEN_HEIGHT: u32 = 800;
const PLAYER_SPEED: u32 = 5;

struct Player {
    pos_x: u32,
    pos_y: u32,
}

fn move_player(rl: &RaylibHandle, player: &mut Player) {
    if rl.is_key_down(KeyboardKey::KEY_RIGHT) {
        player.pos_x += PLAYER_SPEED;
    }
    if rl.is_key_down(KeyboardKey::KEY_LEFT) {
        player.pos_x -= PLAYER_SPEED;
    }
    if rl.is_key_down(KeyboardKey::KEY_UP) {
        player.pos_y -= PLAYER_SPEED;
    }
    if rl.is_key_down(KeyboardKey::KEY_DOWN) {
        player.pos_y += PLAYER_SPEED;
    }
}

fn draw_player(d: &mut RaylibDrawHandle, player: &Player) {
    d.draw_circle(player.pos_x as i32, player.pos_y as i32, 10.0, Color::RED);
}

fn main() {
    let (mut rl, thread) = raylib::init().size(600, 800).title("banan").build();
    rl.set_target_fps(60);

    let mut player = Player {
        pos_x: SCREEN_WIDTH / 2,
        pos_y: SCREEN_HEIGHT / 2,
    };

    while !rl.window_should_close() {
        move_player(&rl, &mut player);

        let mut d: RaylibDrawHandle = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        draw_player(&mut d, &player);
    }
}
