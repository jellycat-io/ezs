use std::ops::{Add, AddAssign, MulAssign, Sub};

use rand::{thread_rng, Rng};

#[derive(Debug, Copy, Clone)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

impl Vector2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn zero() -> Self {
        Self { x: 0.0, y: 0.0 }
    }

    pub fn new_random_range(min: f32, max: f32) -> Self {
        let mut rng = thread_rng();
        let x = rng.gen_range(min..max);
        let y = rng.gen_range(min..max);

        Self { x, y }
    }

    pub fn to_mint_vector2(self) -> ggez::mint::Vector2<f32> {
        ggez::mint::Vector2 {
            x: self.x,
            y: self.y,
        }
    }

    /// Get the magnitude of the vector.
    pub fn magnitude(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

    pub fn distance_to(&self, other: &Self) -> f32 {
        (*self - other).magnitude()
    }

    /// Normalize the vector by setting its magnitude to 1
    pub fn normalize(&mut self) {
        let magnitude = self.magnitude();
        self.x /= magnitude;
        self.y /= magnitude;
    }

    pub fn reset(&mut self) {
        self.x = 0.0;
        self.y = 0.0;
    }

    pub fn reset_x(&mut self) {
        self.x = 0.0;
    }

    pub fn reset_y(&mut self) {
        self.y = 0.0;
    }
}

impl Add for Vector2 {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl AddAssign for Vector2 {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl Sub<&Vector2> for Vector2 {
    type Output = Vector2;

    fn sub(self, other: &Vector2) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl MulAssign<f32> for Vector2 {
    fn mul_assign(&mut self, factor: f32) {
        self.x *= factor;
        self.y *= factor;
    }
}
