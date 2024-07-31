use raylib::{
    color::Color,
    drawing::{RaylibDraw, RaylibDrawHandle},
    ffi::{KeyboardKey, Rectangle, Vector2},
    RaylibHandle,
};

use crate::consts::Consts;

pub struct Player {
    pub x: i32,
    pub y: i32,
    rotation: f32,
}

impl Player {
    pub const SPEED: i32 = 3;
    pub const SIZE: i32 = 30;

    pub fn new() -> Self {
        Player {
            rotation: 0.0,
            x: Consts::SCREEN_WIDTH / 2,
            y: Consts::SCREEN_HEIGHT / 2,
        }
    }

    pub fn movement(&mut self, rl: &RaylibHandle) {
        if rl.is_key_down(KeyboardKey::KEY_RIGHT) {
            self.x += Player::SPEED;
        }
        if rl.is_key_down(KeyboardKey::KEY_LEFT) {
            self.x -= Player::SPEED;
        }
        if rl.is_key_down(KeyboardKey::KEY_UP) {
            self.y -= Player::SPEED;
        }
        if rl.is_key_down(KeyboardKey::KEY_DOWN) {
            self.y += Player::SPEED;
        }

        if self.x + (Player::SIZE / 2) > Consts::SCREEN_WIDTH {
            self.x = Consts::SCREEN_WIDTH - (Player::SIZE / 2);
        } else if self.x < (Player::SIZE / 2) {
            self.x = Player::SIZE / 2;
        }

        if self.y + (Player::SIZE / 2) > Consts::SCREEN_HEIGHT {
            self.y = Consts::SCREEN_HEIGHT - (Player::SIZE / 2);
        } else if self.y < (Player::SIZE / 2) {
            self.y = Player::SIZE / 2;
        }
    }

    pub fn draw(&mut self, d: &mut RaylibDrawHandle) {
        d.draw_rectangle_pro(
            Rectangle {
                x: self.x as f32,
                y: self.y as f32,
                width: Player::SIZE as f32,
                height: Player::SIZE as f32,
            },
            Vector2 {
                x: (Player::SIZE / 2) as f32,
                y: (Player::SIZE / 2) as f32,
            },
            self.rotation,
            Color::ORANGE,
        );

        self.rotation += 5.0;
    }
}
