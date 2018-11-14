use ray::Ray;
use vec3::Vec3;
use hitable::*;

pub struct Sphere {
	center: Vec3,
	radius: f32
}

impl Sphere {
	pub fn new(cen: Vec3, r: f32) -> Sphere {
		Sphere { center: cen, radius: r }
	}
}

impl Hitable for Sphere {
	fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
		let oc = r.origin() - &self.center;
		let a = Vec3::dot(r.direction(), r.direction());
		let b = Vec3::dot(&oc, r.direction());
		let c = Vec3::dot(&oc, &oc) - self.radius*self.radius;
		let discriminant = b*b - a*c;
		let closure_record = |sqrt_num| {
			let t = sqrt_num;
			let p = r.point_at_parameter(t);
			let normal = (p - self.center) / self.radius;
			return Some(HitRecord {t, p, normal});
		};
		if discriminant > 0.0 {
			let temp = (-b - (b*b - a*c).sqrt()) / a;
			if temp < t_max && temp > t_min {
				return closure_record(temp);
			}
			let temp = (-b + (b*b - a*c).sqrt()) / a;
			if temp < t_max && temp > t_min {
				return closure_record(temp);
			}
		}
		None
	}
}