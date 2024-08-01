use raylib::{color::Color, drawing::RaylibDrawHandle, ffi::KeyboardKey, RaylibHandle};

use super::game_object::GameObject;
use crate::consts::Consts;

pub struct Player {
    game_object: GameObject,
}

impl Player {
    pub const SPEED: i32 = 3;
    pub const SIZE: i32 = 30;

    pub fn new() -> Self {
        Player {
            game_object: GameObject::new(
                Consts::SCREEN_WIDTH / 2,
                Consts::SCREEN_HEIGHT / 2,
                Player::SIZE,
                Player::SIZE,
                0.0,
                None,
                Some(Color::ORANGE),
            ),
        }
    }

    pub fn movement(&mut self, rl: &RaylibHandle) {
        if rl.is_key_down(KeyboardKey::KEY_RIGHT) {
            self.game_object.x += Player::SPEED;
        }
        if rl.is_key_down(KeyboardKey::KEY_LEFT) {
            self.game_object.x -= Player::SPEED;
        }
        if rl.is_key_down(KeyboardKey::KEY_UP) {
            self.game_object.y -= Player::SPEED;
        }
        if rl.is_key_down(KeyboardKey::KEY_DOWN) {
            self.game_object.y += Player::SPEED;
        }

        if self.game_object.x + (Player::SIZE / 2) > Consts::SCREEN_WIDTH {
            self.game_object.x = Consts::SCREEN_WIDTH - (Player::SIZE / 2);
        } else if self.game_object.x < (Player::SIZE / 2) {
            self.game_object.x = Player::SIZE / 2;
        }

        if self.game_object.y + (Player::SIZE / 2) > Consts::SCREEN_HEIGHT {
            self.game_object.y = Consts::SCREEN_HEIGHT - (Player::SIZE / 2);
        } else if self.game_object.y < (Player::SIZE / 2) {
            self.game_object.y = Player::SIZE / 2;
        }
    }

    pub fn draw(&mut self, d: &mut RaylibDrawHandle) {
        self.game_object.draw(d);

        self.game_object.rotation += 5.0;
    }
}
