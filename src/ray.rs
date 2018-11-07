use vec3::Vec3;

pub struct Ray {
	pub a : Vec3<f32>,
	pub b : Vec3<f32>
}

impl Ray {
	pub fn origin(&self) -> &Vec3<f32> {
        &self.a
    }

    pub fn direction(&self) -> &Vec3<f32> {
        &self.b
    }

    pub fn point_at_parameter(&self, t: f32) -> Vec3<f32> {
    	Vec3 { a: self.a.x() + t * self.b.x(), b: self.a.y() + t * self.b.y(), c: self.a.z() + t * self.b.z() }
    }
}