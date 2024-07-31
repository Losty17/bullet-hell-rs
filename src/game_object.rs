use raylib::{
    color::Color,
    drawing::{RaylibDraw, RaylibDrawHandle},
    texture::Texture2D,
};

pub struct GameObject {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    texture: Texture2D,
}

impl GameObject {
    fn new(x: i32, y: i32, width: i32, height: i32, texture: Texture2D) -> Self {
        Self {
            x,
            y,
            width,
            height,
            texture,
        }
    }

    fn draw(&self, d: &mut RaylibDrawHandle) {
        d.draw_texture(&self.texture, self.x, self.y, Color::WHITE);
    }
}
