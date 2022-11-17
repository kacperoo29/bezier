use std::ops::{Add, Mul, Sub};

use num::Num;

pub type Vector2f = Vector<f32, 2>;

#[derive(Debug, Clone, Copy)]
pub struct Vector<T: Num + Copy, const N: usize> {
    data: [T; N],
}

impl<T: Num + Copy> Vector<T, 2> {
    pub fn new_with_data(x: T, y: T) -> Self {
        Self { data: [x, y] }
    }

    pub fn x(&self) -> T {
        self.data[0]
    }

    pub fn y(&self) -> T {
        self.data[1]
    }

    pub fn set_x(&mut self, x: T) {
        self.data[0] = x;
    }

    pub fn set_y(&mut self, y: T) {
        self.data[1] = y;
    }
}

impl Vector<f32, 2> {
    pub fn distance_to(&self, other: Vector<f32, 2>) -> f32 {
        let x = self.x() - other.x();
        let y = self.y() - other.y();
        (x * x + y * y).sqrt()
    }
}

impl<T: Num + Copy> Add<Self> for Vector<T, 2> {
    type Output = Self;

    fn add(self, other: Vector<T, 2>) -> Self {
        Self {
            data: [self.data[0] + other.data[0], self.data[1] + other.data[1]],
        }
    }
}

impl<T: Num + Copy> Sub<Self> for Vector<T, 2> {
    type Output = Self;

    fn sub(self, other: Vector<T, 2>) -> Self {
        Self {
            data: [self.data[0] - other.data[0], self.data[1] - other.data[1]],
        }
    }
}

impl<T: Num + Copy> Mul<T> for Vector<T, 2> {
    type Output = Self;

    fn mul(self, other: T) -> Self {
        Self {
            data: [self.data[0] * other, self.data[1] * other],
        }
    }
}
