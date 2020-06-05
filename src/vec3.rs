use std::ops::{Add, Deref, DerefMut, Div, Mul, Sub, SubAssign, Neg};

pub type Color = Vec3;
pub type Point = Vec3;

#[derive(Clone, Copy)]
pub struct Vec3([f64; 3]);

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self([x, y, z])
    }

    pub fn length_squared(&self) -> f64 {
        self.iter().map(|e| e * e).sum()
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn unit(&self) -> Vec3 {
        *self / self.length()
    }

    pub fn x(&self) -> f64 {
        self[0]
    }

    pub fn y(&self) -> f64 {
        self[1]
    }

    pub fn z(&self) -> f64 {
        self[2]
    }
}

pub fn dot(a: Vec3, b: Vec3) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

impl Deref for Vec3 {
    type Target = [f64; 3];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Vec3 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

// Division by scalar
impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self {
        Self::new(self[0] / rhs, self[1] / rhs, self[2] / rhs)
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self {
        Self::new(self[0] * rhs, self[1] * rhs, self[2] * rhs)
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Self {
        Self::new(self[0] - rhs[0], self[1] - rhs[1], self[2] - rhs[2])
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self {
        Self::new(self[0] + rhs[0], self[1] + rhs[1], self[2] + rhs[2])
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        for i in 0..3 {
            self[i] -= rhs[i]
        }
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        self * (-1.)
    }
}
