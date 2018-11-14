extern crate image;

use std::fs::File;
use std::path::Path;
use std::f32;

mod vec3;
use vec3::Vec3;

mod ray;
use ray::Ray;

mod hitable;
use hitable::*;
mod sphere;
use sphere::Sphere;

mod hitablelist;
use hitablelist::HitableList;

fn color(r: &Ray, world: &Hitable) -> Vec3 {
    let hitting = world.hit(r, 0.0, f32::MAX);
    match hitting {
        Some(hit_record) => {
            return Vec3::new(hit_record.normal.x() + 1.0, hit_record.normal.y() + 1.0, hit_record.normal.z() + 1.0) * 0.5;
        },
        None => {
            let unit_direction = Vec3::unit_vector(r.direction());
            let t = 0.5 * (unit_direction.y() + 1.0);
            return Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t;
        }
    }
}

fn main() -> std::io::Result<()>
{
	const NX : usize = 200;
	const NY : usize = 100;
    let mut pixels : Vec<u8> = vec![0; NX * NY * 3];
    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);
    let mut world = HitableList { list: Vec::new()};
    world.list.push(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)));
    world.list.push(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)));
    for j in 0..NY {
		for i in 0..NX {
            let u = i as f32 / NX as f32;
            let v = j as f32 / NY as f32;
            let r = Ray::new(&origin, &(lower_left_corner + horizontal * u + vertical * v));
            let _p = r.point_at_parameter(2.0);
            let col = color(&r, &world);
            let ir = (255.99*col.r()) as u8;
            let ig = (255.99*col.g()) as u8;
            let ib = (255.99*col.b()) as u8;
            pixels[i*3 + (NY-j-1)*NX*3] = ir;
            pixels[1 + i*3 + (NY-j-1)*NX*3] = ig;
            pixels[2 + i*3 + (NY-j-1)*NX*3] = ib;
		}
	}

    write_image("OutputImage.png", &pixels, NX, NY)?;;

    Ok(())
}

fn write_image(filepath: &str, pixels: &[u8], x_size: usize, y_size: usize) -> std::io::Result<()>
{
    let image_path = Path::new(filepath);
    let file = File::create(&image_path)?;
    let png_encoder = image::png::PNGEncoder::new(file);
    png_encoder.encode(&pixels, x_size as u32, y_size as u32, image::ColorType::RGB(8))?;
    Ok(())
}