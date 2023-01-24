use crate::v2d::Vector2D;
use macroquad::prelude::{Color, draw_circle};

pub struct Blob
{
    pub position: Vector2D,
    velocity: Vector2D,
    pub size: f32,
    pub color: Color
}

impl Blob
{
    pub fn from_xy(x: f32, y: f32, color: Color) -> Blob
    {
        return Blob
        {
            position: Vector2D::from_xy(x, y),
            velocity: Vector2D::new(),
            size: 0.1,
            color: color
        };
    }


    pub fn draw(&self)
    {
        draw_circle(self.position.x, self.position.y, self.size, self.color);
    }

    pub fn update(&mut self, dt: f32, target: Vector2D)
    {
        self.velocity = (target - self.position);
        if self.velocity.length() == 0.0
        {
            return;
        }
        self.velocity.normalize();
        self.velocity = self.velocity * (0.2 - self.size / 1000.0);


        self.position = self.position + self.velocity * dt;
    }

}



