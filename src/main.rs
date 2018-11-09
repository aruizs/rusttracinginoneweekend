extern crate image;

use std::fs::File;
use std::path::Path;

mod vec3;
use vec3::Vec3;

mod ray;
use ray::Ray;

fn hit_sphere(center: &Vec3, radius: f32, r: &Ray) -> bool {
    let oc = r.origin() - center;
    let a = Vec3::dot(r.direction(), r.direction());
    let b = 2.0 * Vec3::dot(&oc, &r.direction());
    let c = Vec3::dot(&oc, &oc) - radius*radius;
    let discriminant = b*b - 4.0*a*c;
    discriminant > 0.0
}

fn color(r: &Ray) -> Vec3 {
    if hit_sphere(&Vec3::new(0.0, 0.0, -1.0), 0.5, &r)
    {
        return Vec3::new(1.0, 0.0, 0.0)
    }
    let unit_direction = Vec3::unit_vector(r.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t 
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
	for j in 0..NY {
		for i in 0..NX {
            let u = i as f32 / NX as f32;
            let v = j as f32 / NY as f32;
            let r = Ray::new(&origin, &(lower_left_corner + horizontal * u + vertical * v));
            let col = color(&r);
            let ir = (255.99*col.r()) as u8;
            let ig = (255.99*col.g()) as u8;
            let ib = (255.99*col.b()) as u8;
            pixels[i*3 + (NY-j-1)*NX*3] = ir;
            pixels[1 + i*3 + (NY-j-1)*NX*3] = ig;
            pixels[2 + i*3 + (NY-j-1)*NX*3] = ib;
		}
	}

    write_image("out/archivo.png", &pixels, NX, NY)?;;

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