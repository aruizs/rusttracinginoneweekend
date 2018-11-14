use ray::Ray;
use hitable::*;

pub struct HitableList 
{
	pub list: Vec<Box<Hitable>>
}

impl Hitable for HitableList {
	fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
		let mut closest_so_far = t_max;
		let mut hit_record: Option<HitRecord> = None;
		for item in self.list.iter() {
			let hitting = item.hit(r, t_min, closest_so_far);
			match hitting {
				Some(hit_result) => {
					closest_so_far = hit_result.t;
					hit_record = Some(hit_result);
				},
				None => {
				}
			}
		}
		hit_record
	}
}