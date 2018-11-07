extern crate image;

use std::fs::File;
use std::path::Path;

mod vec3;
use vec3::Vec3;

mod ray;
use ray::Ray;

fn main() -> std::io::Result<()>
{
	const NX : usize = 200;
	const NY : usize = 100;
    let mut pixels : Vec<u8> = vec![0; NX * NY * 3];
	for i in 0..NX {
		for j in 0..NY {
            let col = Vec3::new(i as f32 / NX as f32, j as f32 / NY as f32, 0.2);
            let ir = (255.99*col.r()) as u8;
            let ig = (255.99*col.g()) as u8;
            let ib = (255.99*col.b()) as u8;
            pixels[i*3 + j*NX*3] = ir;
            pixels[1 + i*3 + j*NX*3] = ig;
            pixels[2 + i*3 + j*NX*3] = ib;
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