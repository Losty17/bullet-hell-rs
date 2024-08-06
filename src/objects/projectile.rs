use raylib::{
    color::Color,
    drawing::{RaylibDraw, RaylibDrawHandle},
};

use crate::consts::Consts;

use super::game_object::GameObject;

pub struct Projectile {
    pub game_object: GameObject,
    speed: f32,
    velocity: f32,
    acceleration: f32,
    offset: f32,
    min_speed: Option<f32>,
    max_speed: Option<f32>,
    direction: f32,
    f_x: f32,
    f_y: f32,
    target: Option<GameObject>,
}

impl Projectile {
    pub fn new(x: i32, y: i32, width: i32, height: i32, target: Option<&GameObject>) -> Self {
        let mut direction = 0.0;
        match target {
            Some(ref t) => {
                let dx = t.x as f32 - x as f32;
                let dy = t.y as f32 - y as f32;
                direction = dy.atan2(dx);
            }
            None => {}
        };
        
        let projectile = Self {
            game_object: GameObject::new(x, y, width, height, 0.0, None, Some(Color::GREEN)),
            velocity: 0.0,
            acceleration: 0.04,
            offset: 0.0,
            min_speed: Some(4.0),
            max_speed: Some(8.0),
            direction,
            speed: 0.0,
            f_x: x as f32,
            f_y: y as f32,
            target: None,
        };


        projectile
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        d.draw_circle(
            self.game_object.x,
            self.game_object.y,
            self.game_object.width as f32 / 2.0,
            self.game_object.color,
        )
    }

    pub fn update(&mut self) {
        self.direction += self.offset;

        if self.min_speed.is_some()
            && self.velocity + self.acceleration >= self.min_speed.unwrap()
            && self.max_speed.is_some()
            && self.velocity + self.acceleration <= self.max_speed.unwrap()
        {}

        self.velocity += self.acceleration;
        self.f_x += (self.speed + self.velocity) * (self.direction).cos();
        self.f_y += (self.speed + self.velocity) * (self.direction).sin();

        self.game_object.x = self.f_x as i32;
        self.game_object.y = self.f_y as i32;
    }
}
