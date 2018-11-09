use vec3::Vec3;

pub struct Ray {
	pub a : Vec3,
    pub b : Vec3
}

impl Ray {

	pub fn new(a_in: &Vec3, b_in: &Vec3) -> Ray 	{
		Ray { a: a_in.clone(), b: b_in.clone() }
	}

	pub fn origin(&self) -> &Vec3 {
        &self.a
    }

    pub fn direction(&self) -> &Vec3 {
        &self.b
    }

    pub fn point_at_parameter(&self, t: f32) -> Vec3 {
    	Vec3 { a: self.a.x() + t * self.b.x(), b: self.a.y() + t * self.b.y(), c: self.a.z() + t * self.b.z() }
    }
}