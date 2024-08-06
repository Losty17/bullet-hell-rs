use raylib::{color::Color, drawing::RaylibDrawHandle, ffi::PI, RaylibHandle};

use crate::objects::{game_object::GameObject, Projectile};

pub struct TestEnemy {
    game_object: GameObject,
}

impl TestEnemy {
    pub fn new(x: i32, y: i32, width: i32, height: i32, rotation: f32) -> Self {
        Self {
            game_object: GameObject::new(x, y, width, height, rotation, None, Some(Color::RED)),
        }
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        self.game_object.draw(d);
    }

    pub fn update(&mut self, rl: &RaylibHandle, target: Option<&GameObject>) -> Vec<Projectile> {
        let mut projectiles: Vec<Projectile> = vec![];

        for angle in (-90..100).step_by(10) {
            let radians = 2.0 * PI * (angle as f64) / 360.0;

            let base_x = 120.0 * radians.sin();
            let base_y = 120.0 * radians.cos();

            let x = base_x + self.game_object.x as f64;
            let y = base_y + self.game_object.y as f64;

            let p = Projectile::new(x as i32, y as i32, 10, 20, target);

            projectiles.push(p);
        }

        projectiles
    }
}
