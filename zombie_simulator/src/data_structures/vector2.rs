use rand::{thread_rng, Rng};
use std::ops::{Add, AddAssign, Sub};

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

    pub fn to_mint_vector2(&self) -> ggez::mint::Vector2<f32> {
        ggez::mint::Vector2 {
            x: self.x,
            y: self.y,
        }
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
    type Output = Vector2;

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

impl Sub for Vector2 {
    type Output = Vector2;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}
