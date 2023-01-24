use macroquad::prelude::{GREEN, draw_circle};
use macroquad::rand;

use crate::v2d::Vector2D;

const ArraySize: usize = 2000;

pub struct FoodArray
{
    foods: [Food; ArraySize]
}

impl FoodArray
{
    pub fn new() -> FoodArray
    {
        let mut array: [Food; ArraySize] = [Food::from_xy(0.0, 0.0); ArraySize];

        for i in 0..array.len()
        {
            let x = rand::gen_range(-15.0, 15.0);
            let y = rand::gen_range(-15.0, 15.0);

            array[i].position.x = x;
            array[i].position.y = y;
        }

        return FoodArray
        {
            foods: array
        };
    }


    pub fn draw(&self)
    {
        for food in &self.foods
        {
            food.draw();
        }
    }
}











#[derive(Copy, Clone)]
pub struct Food
{
    position: Vector2D
}

impl Food
{
    pub fn from_xy(x: f32, y: f32) -> Food
    {
        return Food
        {
            position: Vector2D::from_xy(x, y)
        };
    }

    pub fn draw(&self)
    {
        draw_circle(self.position.x, self.position.y, 0.02, GREEN);
    }
}