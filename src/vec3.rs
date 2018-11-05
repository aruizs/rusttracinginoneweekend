pub struct Vec3<T> {
    pub a: T,
    pub b: T,
    pub c: T
}

impl<T> Vec3<T> {
    pub fn new(x: T, y: T, z: T) -> Vec3<T> {
        Vec3 { a: x, b: y, c: z}
    }

    pub fn x(&self) -> &T {
        &self.a
    }

    pub fn y(&self) -> &T {
        &self.b
    }

    pub fn z(&self) -> &T {
        &self.c
    }

    pub fn r(&self) -> &T {
        &self.a
    }

    pub fn g(&self) -> &T {
        &self.b
    }

    pub fn b(&self) -> &T {
        &self.c
    }
}