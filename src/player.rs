use raylib::{
    color::Color,
    drawing::{RaylibDraw, RaylibDrawHandle},
    ffi::KeyboardKey,
    RaylibHandle,
};

use crate::consts::Consts;

pub struct Player {
    pub x: i32,
    pub y: i32,
}

impl Player {
    pub const SPEED: i32 = 3;
    pub const SIZE: i32 = 20;

    pub fn new() -> Self {
        Player {
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

        if self.x + Player::SIZE > Consts::SCREEN_WIDTH {
            self.x = Consts::SCREEN_WIDTH - 20;
        } else if self.x < 0 {
            self.x = 0;
        }

        if self.y + Player::SIZE > Consts::SCREEN_HEIGHT {
            self.y = Consts::SCREEN_HEIGHT - 20;
        } else if self.y < 0 {
            self.y = 0;
        }
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        d.draw_rectangle(self.x, self.y, 20, 20, Color::RED);
    }
}
