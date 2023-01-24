use std::ops;
use std::fmt;

#[derive(Copy, Clone, PartialEq)]
pub struct Vector2D
{
    pub x: f32,
    pub y: f32
}

//CONSTRUCTORS
impl Vector2D
{
    pub fn from_xy(x: f32, y: f32) -> Vector2D
    {
        return Vector2D
        {
            x: x,
            y: y
        };
    }

    pub fn from_xy_i32(x: i32, y: i32) -> Vector2D
    {
        return Vector2D
        {
            x: x as f32,
            y: y as f32
        };
    }

    pub fn new() -> Vector2D
    {
        return Vector2D
        {
            x: 0.0,
            y: 0.0
        };
    }

    pub fn clone(&self) -> Vector2D
    {
        return Vector2D
        {
            x: self.x,
            y: self.y
        };
    }
}

//DISPLAY
impl fmt::Display for Vector2D
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "({}, {})", self.x, self.y)
    }
}

//OPERATOR OVERLOADS
// + - += -= * *= / ==
impl ops::Add for Vector2D // +
{
    type Output = Vector2D;
    fn add(self, rhs: Self) -> Vector2D
    {
        return Vector2D
        {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        };
    }
}

impl ops::AddAssign for Vector2D // +=
{
    fn add_assign(&mut self, rhs: Self)
    {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl ops::Sub for Vector2D // -
{
    type Output = Vector2D;
    fn sub(self, rhs: Self) -> Vector2D
    {
        return Vector2D
        {
            x: self.x - rhs.x,
            y: self.y - rhs.y
        };
    }
}

impl ops::SubAssign for Vector2D // -=
{
    fn sub_assign(&mut self, rhs: Self)
    {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl ops::Mul<f32> for Vector2D // * f32
{
    type Output = Vector2D;
    fn mul(self, rhs: f32) -> Vector2D
    {
        return Vector2D
        {
            x: self.x * rhs,
            y: self.y * rhs
        };
    }
}

impl ops::MulAssign<f32> for Vector2D // *= f32
{
    fn mul_assign(&mut self, rhs: f32)
    {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl ops::Mul<i32> for Vector2D // * i32
{
    type Output = Vector2D;
    fn mul(self, rhs: i32) -> Vector2D
    {
        return Vector2D
        {
            x: self.x * rhs as f32,
            y: self.y * rhs as f32
        };
    }
}

impl ops::MulAssign<i32> for Vector2D // *= i32
{
    fn mul_assign(&mut self, rhs: i32)
    {
        self.x *= rhs as f32;
        self.y *= rhs as f32;
    }
}

impl ops::Mul<Vector2D> for Vector2D // dot product
{
    type Output = f32;
    fn mul(self, rhs: Self) -> f32
    {
        return self.x * rhs.x + self.y * rhs.y;
    }
}

impl ops::Div<f32> for Vector2D // division f32
{
    type Output = Vector2D;
    fn div(self, rhs: f32) -> Vector2D
    {
        return Vector2D
        {
            x: self.x / rhs,
            y: self.y / rhs
        };
    }
}

impl ops::Div<i32> for Vector2D //division i32
{
    type Output = Vector2D;
    fn div(self, rhs: i32) -> Vector2D
    {
        return Vector2D
        {
            x: self.x / rhs as f32,
            y: self.y / rhs as f32
        };
    }
}


//VECTOR MATH
impl Vector2D
{
    pub fn length(&self) -> f32
    {
        return (self.x.powf(2.0) + self.y.powf(2.0)).sqrt();
    }

    pub fn normalize(&mut self)
    {
        self.x /= self.length();
        self.y /= self.length();
    }




    pub fn distance(v1: Vector2D, v2: Vector2D) -> f32
    {
        return (v1 - v2).length();
    }
}