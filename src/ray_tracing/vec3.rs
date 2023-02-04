use std::ops::{Add, Sub};

pub struct Vec3 {
    pub a: i32,
    pub b: i32,
    pub c: i32,
}

impl Vec3 {
    pub fn new(a: i32, b: i32, c: i32) -> Self {
        Self {
            a, b, c
        }
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            a: self.a + rhs.a,
            b: self.b + rhs.b,
            c: self.c + rhs.c
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            a: self.a - rhs.a,
            b: self.b - rhs.b,
            c: self.c - rhs.c
        }
    }
}