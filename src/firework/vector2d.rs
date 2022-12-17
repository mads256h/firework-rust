use std::ops::{Add, Mul, Sub};

#[derive(Copy, Clone)]
pub struct Vector2D {
    pub x: f32,
    pub y: f32,
}

impl Vector2D {
    pub fn new(x: f32, y: f32) -> Self {
        Vector2D { x, y }
    }
}

impl Add<Vector2D> for Vector2D {
    type Output = Self;

    fn add(self, rhs: Vector2D) -> Self::Output {
        Vector2D::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Sub<Vector2D> for Vector2D {
    type Output = Self;

    fn sub(self, rhs: Vector2D) -> Self::Output {
        Vector2D::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl Mul<f32> for Vector2D {
    type Output = Vector2D;

    fn mul(self, rhs: f32) -> Self::Output {
        Vector2D::new(self.x * rhs, self.y * rhs)
    }
}
