use raylib::{
    color::Color,
    drawing::{RaylibDraw, RaylibDrawHandle},
    math::{Rectangle, Vector2},
    texture::Texture2D,
};

pub(super) struct GameObject {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub rotation: f32,
    pub texture: Option<Texture2D>,
    pub color: Color,
}

impl GameObject {
    pub(super) fn new(
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        rotation: f32,
        texture: Option<Texture2D>,
        _color: Option<Color>,
    ) -> Self {
        let mut color = Color::WHITE;

        if let Some(c) = _color {
            color = c;
        }

        Self {
            x,
            y,
            width,
            height,
            texture,
            rotation,
            color,
        }
    }

    pub(super) fn draw(&self, d: &mut RaylibDrawHandle) {
        match &self.texture {
            Some(texture) => {
                let rec = Rectangle {
                    x: self.x as f32,
                    y: self.y as f32,
                    width: self.width as f32,
                    height: self.width as f32,
                };

                d.draw_texture_pro(
                    &texture,
                    rec,
                    rec,
                    Vector2 {
                        x: (self.width / 2) as f32,
                        y: (self.height / 2) as f32,
                    },
                    self.rotation,
                    Color::WHITE,
                );
            }
            None => {
                d.draw_rectangle_pro(
                    Rectangle {
                        x: self.x as f32,
                        y: self.y as f32,
                        width: self.width as f32,
                        height: self.width as f32,
                    },
                    Vector2 {
                        x: (self.width / 2) as f32,
                        y: (self.height / 2) as f32,
                    },
                    self.rotation,
                    self.color,
                );
            }
        }
    }
}
