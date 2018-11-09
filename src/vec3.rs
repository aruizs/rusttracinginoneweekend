use std::ops::{Add, Mul, Div};

#[derive(Copy, Clone, Debug)]
pub struct Vec3 {
    pub a: f32,
    pub b: f32,
    pub c: f32
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3 { a: self.a + other.a, b: self.b + other.b, c: self.c + other.c}
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, factor: f32) -> Vec3 {
        Vec3 { a: factor * self.a, b: factor * self.b, c: factor * self.c}
    }
}

impl Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, factor: f32) -> Vec3 {
        Vec3 { a: self.a / factor, b: self.b / factor, c: self.c / factor}
    }
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { a: x, b: y, c: z}
    }

    pub fn x(&self) -> &f32 {
        &self.a
    }

    pub fn y(&self) -> &f32 {
        &self.b
    }

    pub fn z(&self) -> &f32 {
        &self.c
    }

    pub fn r(&self) -> &f32 {
        &self.a
    }

    pub fn g(&self) -> &f32 {
        &self.b
    }

    pub fn b(&self) -> &f32 {
        &self.c
    }

    pub fn length(&self) -> f32 {
        (self.a * self.a + self.b * self.b + self.c * self.c).sqrt()
    }

    pub fn unit_vector(v: &Vec3) -> Vec3 {
        let vector = v.clone();
        vector / vector.length()
    }
}