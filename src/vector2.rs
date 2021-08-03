use std::ops::{ Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign };

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vector2
{
    pub x : f32,
    pub y : f32
}

impl Vector2
{
    pub fn New(_x : f32, _y : f32) -> Self
    {
        Vector2
        {
            x : _x,
            y : _y
        }
    }

    pub fn Dot(a : &Vector2, b : &Vector2) -> f32
    {
        a.x * b.x + a.x * b.y
    }
}

impl Add<Vector2> for Vector2
{
    type Output = Self;

    fn add(self, other : Self) -> Self
    {
        Self
        {
            x : self.x + other.x,
            y : self.y + other.y
        }
    }
}

impl Sub<Vector2> for Vector2
{
    type Output = Self;

    fn sub(self, other : Self) -> Self
    {
        Self
        {
            x : self.x - other.x,
            y : self.y - other.y
        }
    }
}

impl Mul<f32> for Vector2
{
    type Output = Self;

    fn mul(self, other : f32) -> Self
    {
        Self
        {
            x : self.x * other,
            y : self.y * other
        }
    }
}

impl Div<f32> for Vector2
{
    type Output = Self;

    fn div(self, other : f32) -> Vector2
    {
        Self
        {
            x : self.x / other,
            y : self.y / other
        }
    }
}

impl AddAssign<Vector2> for Vector2
{
    fn add_assign(&mut self, other : Self)
    {
        *self = Self
        {
            x: self.x + other.x,
            y: self.y + other.y
        };
    }
}

impl SubAssign<Vector2> for Vector2
{
    fn sub_assign(&mut self, other : Self)
    {
        *self = Self
        {
            x : self.x - other.x,
            y : self.y - other.y
        };
    }
}

impl MulAssign<f32> for Vector2
{
    fn mul_assign(&mut self, other : f32)
    {
        *self = Self
        {
            x: self.x * other,
            y: self.y * other
        };
    }
}

impl DivAssign<f32> for Vector2
{
    fn div_assign(&mut self, other : f32)
    {
        *self = Self
        {
            x : self.x / other,
            y : self.y / other
        };
    }
}